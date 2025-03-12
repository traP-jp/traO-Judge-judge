use crate::model::{identifiers::RuntimeId, job, judge_output, procedure::runtime};
use anyhow::Context;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Runner<
    ReservationToken: Send + Sync + 'static,
    OutcomeToken: Clone + Send + Sync + 'static,
    JobApiType: job::JobApi<ReservationToken, OutcomeToken>,
> {
    job_api: JobApiType,
    outcomes: Arc<Mutex<HashMap<RuntimeId, OutcomeToken>>>,
    outputs: Arc<Mutex<HashMap<RuntimeId, judge_output::ExecutionJobResult>>>,
    exec_confs: Arc<Mutex<HashMap<RuntimeId, (ReservationToken, Vec<runtime::Dependency>)>>>,
    file_confs: HashMap<RuntimeId, job::FileConf>,
}

impl<
        ReservationToken: Send + Sync + 'static,
        OutcomeToken: Clone + Send + Sync + 'static,
        JobApiType: job::JobApi<ReservationToken, OutcomeToken>,
    > Runner<ReservationToken, OutcomeToken, JobApiType>
{
    pub async fn new(job_api: JobApiType, procedure: runtime::Procedure) -> anyhow::Result<Self> {
        let file_confs = Self::create_file_confs(&procedure);
        let exec_confs = Self::create_exec_confs(&procedure, &job_api).await?;
        Ok(Self {
            job_api,
            outcomes: Arc::new(Mutex::new(HashMap::new())),
            outputs: Arc::new(Mutex::new(HashMap::new())),
            exec_confs: Arc::new(Mutex::new(exec_confs)),
            file_confs: file_confs,
        })
    }

    pub async fn run(self) -> anyhow::Result<HashMap<RuntimeId, judge_output::ExecutionJobResult>> {
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
                outputs
                    .insert(
                        runtime_id.clone(),
                        judge_output::ExecutionJobResult::EarlyExit,
                    )
                    .context("Failed to insert early exit")?;
            }
            Ok(outputs.clone())
        }
    }

    fn create_file_confs(procedure: &runtime::Procedure) -> HashMap<RuntimeId, job::FileConf> {
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
        file_confs
    }

    async fn create_exec_confs(
        procedure: &runtime::Procedure,
        job_api: &JobApiType,
    ) -> anyhow::Result<HashMap<RuntimeId, (ReservationToken, Vec<runtime::Dependency>)>> {
        let mut reservations_vec = job_api
            .reserve_execution(procedure.executions.len())
            .await
            .context("Failed to reserve executions")?;
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
        Ok(reservations)
    }

    async fn run_file_job(
        &self,
        runtime_id: RuntimeId,
        file_conf: job::FileConf,
    ) -> anyhow::Result<()> {
        let outcome_token = self
            .job_api
            .place_file(file_conf)
            .await
            .context(format!("Failed to place file for runtime {}", runtime_id))?;
        let outcomes = self.new_outcome(runtime_id, outcome_token).await;
        self.run_next(&outcomes).await?;
        Ok(())
    }

    async fn run_execution_job(
        &self,
        runtime_id: RuntimeId,
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> anyhow::Result<()> {
        let (outcome_token, output) = self
            .job_api
            .execute(reservation, dependencies)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let result = judge_output::parse(&output)
            .context(format!("Failed to parse output for {}", runtime_id))?;
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
        Ok(())
    }

    async fn new_outcome(
        &self,
        runtime_id: RuntimeId,
        outcome_token: OutcomeToken,
    ) -> HashMap<RuntimeId, OutcomeToken> {
        let outcomes = {
            let mut outcomes = self.outcomes.lock().await;
            outcomes.insert(runtime_id, outcome_token);
            let cloned: HashMap<RuntimeId, OutcomeToken> = outcomes.clone();
            std::mem::drop(outcomes);
            cloned
        };
        outcomes
    }
    async fn run_next(&self, outcomes: &HashMap<RuntimeId, OutcomeToken>) -> anyhow::Result<()> {
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
        Ok(())
    }
}
