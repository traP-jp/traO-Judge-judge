use crate::model::{identifiers::RuntimeId, job, judge_output, procedure::runtime};
use anyhow::Context;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Runner<
    ReservationToken: Send + Sync + 'static,
    OutcomeToken: Clone + Send + Sync + 'static,
    JobServiceType: job::JobService<ReservationToken, OutcomeToken>,
> {
    job_service: JobServiceType,
    outcomes: Arc<Mutex<HashMap<RuntimeId, OutcomeToken>>>,
    outputs: Arc<Mutex<HashMap<RuntimeId, judge_output::ExecutionJobResult>>>,
    exec_confs: Arc<Mutex<HashMap<RuntimeId, (ReservationToken, Vec<runtime::Dependency>)>>>,
    file_confs: HashMap<RuntimeId, job::FileConf>,
}

impl<
    ReservationToken: Send + Sync + 'static,
    OutcomeToken: Clone + Send + Sync + 'static,
    JobServiceType: job::JobService<ReservationToken, OutcomeToken>,
> Runner<ReservationToken, OutcomeToken, JobServiceType>
{
    pub async fn new(
        job_service: JobServiceType,
        procedure: runtime::Procedure,
    ) -> anyhow::Result<Self> {
        let file_confs = Self::create_file_confs(&procedure);
        let exec_confs = Self::create_exec_confs(&procedure, &job_service).await?;
        Ok(Self {
            job_service,
            outcomes: Arc::new(Mutex::new(HashMap::new())),
            outputs: Arc::new(Mutex::new(HashMap::new())),
            exec_confs: Arc::new(Mutex::new(exec_confs)),
            file_confs: file_confs,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn run(self) -> anyhow::Result<HashMap<RuntimeId, judge_output::ExecutionJobResult>> {
        tracing::info!("Starting the runner");
        {
            let first_futures = {
                let mut first_futures = Vec::new();
                for (runtime_id, file_conf) in self.file_confs.iter() {
                    let future = self.run_file_job(runtime_id.clone(), file_conf.clone());
                    first_futures.push(future);
                }
                first_futures
            };
            if first_futures.is_empty() {
                self.run_next(&HashMap::new()).await?;
            } else {
                futures::future::join_all(first_futures)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;
            }
            let mut outputs = self.outputs.lock().await;
            let exec_confs = self.exec_confs.lock().await;
            for (runtime_id, _) in exec_confs.iter() {
                outputs.insert(
                    runtime_id.clone(),
                    judge_output::ExecutionJobResult::EarlyExit,
                );
            }
            tracing::info!("Runner completed");
            Ok(outputs.clone())
        }
    }

    #[tracing::instrument]
    fn create_file_confs(procedure: &runtime::Procedure) -> HashMap<RuntimeId, job::FileConf> {
        tracing::info!("Creating file configurations");
        let mut file_confs = HashMap::new();
        for text in procedure.texts.iter() {
            let file_conf = job::FileConf::Text(text.resource_id.clone());
            file_confs.insert(text.runtime_id.clone(), file_conf);
        }
        for runtime_text in procedure.runtime_texts.iter() {
            let file_conf = job::FileConf::RuntimeText(runtime_text.content.clone());
            file_confs.insert(runtime_text.runtime_id.clone(), file_conf);
        }
        for empty_directory in procedure.empty_directories.iter() {
            let file_conf = job::FileConf::EmptyDirectory;
            file_confs.insert(empty_directory.runtime_id.clone(), file_conf);
        }
        tracing::info!("File configurations created");
        file_confs
    }

    #[tracing::instrument(skip(job_service))]
    async fn create_exec_confs(
        procedure: &runtime::Procedure,
        job_service: &JobServiceType,
    ) -> anyhow::Result<HashMap<RuntimeId, (ReservationToken, Vec<runtime::Dependency>)>> {
        tracing::info!("Creating execution configurations");
        let mut reservations_vec = job_service
            .reserve_execution(procedure.executions.len())
            .await
            .context("Failed to reserve executions")?;
        tracing::info!("Reserved {} reservations", reservations_vec.len());
        let mut reservations = HashMap::new();
        for execution in procedure.executions.iter() {
            let reservation = reservations_vec
                .pop()
                .context("Failed to reserve execution")?;
            let dependencies = execution
                .dependencies
                .iter()
                .map(|dep| runtime::Dependency {
                    runtime_id: dep.runtime_id.clone(),
                    envvar_name: dep.envvar_name.clone(),
                })
                .collect();
            reservations.insert(execution.runtime_id.clone(), (reservation, dependencies));
        }
        tracing::info!("Execution configurations created");
        Ok(reservations)
    }

    #[tracing::instrument(skip(self))]
    async fn run_file_job(
        &self,
        runtime_id: RuntimeId,
        file_conf: job::FileConf,
    ) -> anyhow::Result<()> {
        tracing::info!("Running file job for {}", runtime_id);
        let outcome_token = self
            .job_service
            .place_file(file_conf)
            .await
            .context(format!("Failed to place file for runtime {}", runtime_id))?;
        tracing::info!("File placed for {}", runtime_id);
        let outcomes = self.new_outcome(runtime_id, outcome_token).await;
        tracing::info!("File job for {} completed", runtime_id);
        self.run_next(&outcomes).await?;
        tracing::info!("Returning from file job for {}", runtime_id);
        Ok(())
    }

    #[tracing::instrument(skip(self, reservation, dependencies))]
    async fn run_execution_job(
        &self,
        runtime_id: RuntimeId,
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> anyhow::Result<()> {
        tracing::info!("Running execution job for {}", runtime_id);
        let (outcome_token, output) = self
            .job_service
            .execute(reservation, dependencies)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        tracing::info!("Execution completed for {}", runtime_id);
        let result = super::output_parser::parse(&output)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Failed to parse output")?;
        tracing::info!("Output parsed for {}", runtime_id);
        if match &result {
            judge_output::ExecutionResult::Displayable(result_inner) => {
                result_inner.continue_status.clone()
            }
            judge_output::ExecutionResult::Hidden(result_inner) => {
                result_inner.continue_status.clone()
            }
        } == judge_output::ContinueStatus::Continue
        {
            let outcomes = self.new_outcome(runtime_id, outcome_token).await;
            self.run_next(&outcomes)
                .await
                .context(format!("Failed to run next job after {}", runtime_id))?;
        }
        {
            let mut outputs = self.outputs.lock().await;
            outputs.insert(
                runtime_id,
                judge_output::ExecutionJobResult::ExecutionResult(result),
            );
            std::mem::drop(outputs);
        }
        tracing::info!("Returning from execution job for {}", runtime_id);
        Ok(())
    }

    async fn new_outcome(
        &self,
        runtime_id: RuntimeId,
        outcome_token: OutcomeToken,
    ) -> HashMap<RuntimeId, OutcomeToken> {
        tracing::info!("Creating new outcome for {}", runtime_id);
        let outcomes = {
            let mut outcomes = self.outcomes.lock().await;
            outcomes.insert(runtime_id, outcome_token);
            let cloned: HashMap<RuntimeId, OutcomeToken> = outcomes.clone();
            std::mem::drop(outcomes);
            cloned
        };
        tracing::info!("New outcome created for {}", runtime_id);
        outcomes
    }

    #[tracing::instrument(skip(self, outcomes))]
    async fn run_next(&self, outcomes: &HashMap<RuntimeId, OutcomeToken>) -> anyhow::Result<()> {
        tracing::info!("Running next jobs");
        let next_job_futures = {
            let mut next_job_futures = Vec::new();
            let mut exec_confs = self.exec_confs.lock().await;
            // Find all jobs which dependencies are all satisfied
            let mut next_job_dependencies = HashMap::new();
            for (runtime_id, (_, dependencies)) in exec_confs.iter() {
                let available_dependencies = dependencies
                    .iter()
                    .map(|dep| {
                        outcomes
                            .get(&dep.runtime_id)
                            .context("Dependency not satisfied yet")
                            .map(|outcome| job::Dependency {
                                envvar: dep.envvar_name.clone(),
                                outcome: outcome.clone(),
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .ok();
                if let Some(available_dependencies) = available_dependencies {
                    next_job_dependencies.insert(runtime_id.clone(), available_dependencies);
                }
            }
            for (runtime_id, dependencies) in next_job_dependencies.into_iter() {
                let (reservation, _) = exec_confs
                    .remove(&runtime_id)
                    .context("Failed to remove exec conf")?;
                let future = self.run_execution_job(runtime_id, reservation, dependencies);
                next_job_futures.push(future);
            }
            std::mem::drop(exec_confs);
            next_job_futures
        };
        futures::future::join_all(next_job_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        tracing::info!("Next jobs completed");
        Ok(())
    }
}
