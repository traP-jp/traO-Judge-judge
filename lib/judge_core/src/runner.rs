use crate::{
    identifiers::RuntimeId,
    job::{
        self, ExecutionJob, ExecutionJobFinished, FilePlacementJob, JobApi,
        JobOutcomeAcquisitionResult, JobOutcomeLink,
    },
    procedure::runtime::Procedure,
};
use futures::{future::join_all, join, Future};
use std::collections::HashMap;
use std::process::Output;
use tokio::sync::broadcast;

pub struct Runner<JobOutcome: Clone, JobApiType: JobApi<JobOutcome>> {
    job_api: JobApiType,
    _phantom: std::marker::PhantomData<JobOutcome>,
}

pub enum ExecutionJobOutput {
    Succeeded(Output),
    FailedExpectedly(Output),
    EarlyExit,
}

#[derive(Debug, thiserror::Error)]
pub enum RunnerRunError {
    #[error("Internal error while running a job: {0}")]
    InternalError(String),
}

impl<JobOutcomeType: Clone, JobApiType: JobApi<JobOutcomeType>> Runner<JobOutcomeType, JobApiType> {
    pub fn new(job_api: JobApiType) -> Self {
        Self {
            job_api,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn run(
        &self,
        procedure: Procedure,
    ) -> Result<HashMap<RuntimeId, ExecutionJobOutput>, RunnerRunError> {
        // Create list of all ids
        let mut file_placement_ids = Vec::new();
        file_placement_ids.extend(procedure.runtime_texts.iter().map(|x| x.runtime_id));
        file_placement_ids.extend(procedure.texts.iter().map(|x| x.runtime_id));
        file_placement_ids.extend(procedure.empty_directories.iter().map(|x| x.runtime_id));
        let execution_ids = procedure
            .executions
            .iter()
            .map(|x| x.runtime_id)
            .collect::<Vec<RuntimeId>>();
        let mut all_ids = file_placement_ids.clone();
        all_ids.extend(execution_ids.clone());
        // Create txs of job outcomes
        let mut outcome_txs = all_ids
            .iter()
            .map(|x| {
                (
                    *x,
                    broadcast::channel::<JobOutcomeAcquisitionResult<JobOutcomeType>>(1).0,
                )
            })
            .collect::<HashMap<_, _>>();
        // Create subscribers list
        let mut outcome_rxs = execution_ids
            .iter()
            .map(|x| (*x, Vec::new()))
            .collect::<HashMap<_, _>>();
        {
            for execution in procedure.executions.iter() {
                for depends_on in execution.depends_on.iter() {
                    let outcome_tx = outcome_txs.remove(&depends_on.runtime_id).ok_or(
                        RunnerRunError::InternalError(format!(
                            "Depends on {} is not found in outcome_txs",
                            depends_on.runtime_id
                        )),
                    )?;
                    let outcome_rx = outcome_tx.subscribe();
                    outcome_rxs
                        .get_mut(&depends_on.runtime_id)
                        .ok_or(RunnerRunError::InternalError(format!(
                            "Execution {} is not found in outcome_rxs",
                            execution.runtime_id
                        )))?
                        .push(outcome_rx);
                    outcome_txs.insert(depends_on.runtime_id, outcome_tx);
                }
            }
        }
        // Separate outcome txs into file placement and execution
        let mut file_placement_outcome_txs = HashMap::new();
        let mut execution_outcome_txs = HashMap::new();
        {
            for (job_id, outcome_tx) in outcome_txs.into_iter() {
                if file_placement_ids.contains(&job_id) {
                    file_placement_outcome_txs.insert(job_id, outcome_tx);
                } else {
                    execution_outcome_txs.insert(job_id, outcome_tx);
                }
            }
        }
        // Create file placement jobs
        let mut file_placement_jobs = HashMap::new();
        {
            for runtime_text in procedure.runtime_texts.iter() {
                let job = FilePlacementJob::PlaceRuntimeTextFile(runtime_text.content.clone());
                file_placement_jobs.insert(runtime_text.runtime_id, job);
            }
            for text in procedure.texts.iter() {
                let job = FilePlacementJob::PlaceTextFile(text.resource_id);
                file_placement_jobs.insert(text.runtime_id, job);
            }
            for empty_directory in procedure.empty_directories.iter() {
                let job = FilePlacementJob::PlaceEmptyDirectory;
                file_placement_jobs.insert(empty_directory.runtime_id, job);
            }
        }
        // Create execution jobs
        let mut execution_jobs = HashMap::new();
        let mut priorities = HashMap::new();
        for execution in procedure.executions.iter() {
            let mut depends_on_with_names = Vec::new();
            for depends_on in execution.depends_on.iter() {
                let outcome_rx = outcome_rxs
                    .get_mut(&depends_on.runtime_id)
                    .ok_or(RunnerRunError::InternalError(format!(
                        "Execution {} is not found in outcome_rxs",
                        execution.runtime_id
                    )))?
                    .pop()
                    .ok_or(RunnerRunError::InternalError(format!(
                        "Outcome rx for {} is not found",
                        depends_on.runtime_id
                    )))?;
                let job_outcome_link = JobOutcomeLink {
                    job_outcome_rx: outcome_rx,
                    envvar_name: depends_on.envvar_name.clone(),
                };
                depends_on_with_names.push(job_outcome_link);
            }
            let job = ExecutionJob {
                script: execution.script.clone(),
                depends_on_with_names,
            };
            execution_jobs.insert(execution.runtime_id, job);
            priorities.insert(execution.runtime_id, execution.priority);
        }
        // Run futures
        let file_placement_job_futures =
            self.file_placement_job_futures(file_placement_jobs, file_placement_outcome_txs)?;
        let execution_job_futures = self
            .execution_job_futures(execution_jobs, execution_outcome_txs, priorities)
            .await?;
        let execution_job_results = self
            .run_all_futures(file_placement_job_futures, execution_job_futures)
            .await?;
        return Ok(execution_job_results);
    }

    pub fn file_placement_job_futures<'a>(
        &'a self,
        file_placement_jobs: HashMap<RuntimeId, FilePlacementJob>,
        mut outcome_txs: HashMap<
            RuntimeId,
            broadcast::Sender<JobOutcomeAcquisitionResult<JobOutcomeType>>,
        >,
    ) -> Result<Vec<impl Future<Output = Result<(), RunnerRunError>> + 'a>, RunnerRunError> {
        // File placement job futures
        let mut file_placement_job_futures = Vec::new();
        for (job_id, job_conf) in file_placement_jobs.into_iter() {
            // Broadcast tx to channels to the JobAPI
            let job_outcome_tx =
                outcome_txs
                    .remove(&job_id)
                    .ok_or(RunnerRunError::InternalError(format!(
                        "Job {} is not found in outcome_txs",
                        job_id
                    )))?;
            // Job API place future
            let place_future = self.job_api.place_file(job_conf);
            // Whole file placement job future
            let job_future = Self::run_place_file_job(place_future, job_outcome_tx);
            file_placement_job_futures.push(job_future);
        }
        return Ok(file_placement_job_futures);
    }

    pub async fn execution_job_futures<'a>(
        &'a self,
        execution_jobs: HashMap<RuntimeId, ExecutionJob<JobOutcomeType>>,
        mut outcome_txs: HashMap<
            RuntimeId,
            broadcast::Sender<JobOutcomeAcquisitionResult<JobOutcomeType>>,
        >,
        priorities: HashMap<RuntimeId, i32>,
    ) -> Result<
        HashMap<RuntimeId, impl Future<Output = Result<ExecutionJobOutput, RunnerRunError>> + 'a>,
        RunnerRunError,
    > {
        // Execution job futures
        let mut execution_job_futures = HashMap::new();
        for (job_id, job_conf) in execution_jobs.into_iter() {
            // Get priority of the job
            let priority = priorities
                .get(&job_id)
                .ok_or(RunnerRunError::InternalError(format!(
                    "Job {} is not found in priorities",
                    job_id
                )))?;
            // Broadcast tx to channels to the JobAPI
            let job_outcome_tx =
                outcome_txs
                    .remove(&job_id)
                    .ok_or(RunnerRunError::InternalError(format!(
                        "Job {} is not found in outcome_txs",
                        job_id
                    )))?;
            // Job API run future
            let run_future = self
                .job_api
                .run_future(job_conf, *priority)
                .await
                .map_err(|e| RunnerRunError::InternalError(e.to_string()))?;
            // Whole execution job future
            let job_future = Self::run_execution_job(run_future, job_outcome_tx);
            execution_job_futures.insert(job_id, job_future);
        }
        return Ok(execution_job_futures);
    }

    pub async fn run_all_futures(
        &self,
        file_placement_job_futures: Vec<impl Future<Output = Result<(), RunnerRunError>>>,
        execution_job_futures: HashMap<
            RuntimeId,
            impl Future<Output = Result<ExecutionJobOutput, RunnerRunError>>,
        >,
    ) -> Result<HashMap<RuntimeId, ExecutionJobOutput>, RunnerRunError> {
        // Run jobs
        let execution_job_results = join!(
            join_all(file_placement_job_futures),
            join_all(
                execution_job_futures
                    .into_iter()
                    .map(|(job_id, job_future)| async move { (job_id, job_future.await) })
            ),
        )
        .1
        .into_iter()
        .map(|(job_id, job_result)| {
            let job_result = job_result.map(|job_outcome| (job_id, job_outcome));
            job_result
        })
        .collect::<Result<HashMap<RuntimeId, ExecutionJobOutput>, RunnerRunError>>()?;
        return Ok(execution_job_results);
    }

    async fn run_place_file_job(
        place_future: impl Future<Output = Result<JobOutcomeType, job::FilePlacementJobError>>,
        outcome_broadcast_tx: broadcast::Sender<JobOutcomeAcquisitionResult<JobOutcomeType>>,
    ) -> Result<(), RunnerRunError> {
        let place_result = place_future.await;
        match place_result {
            Ok(job_outcome) => {
                outcome_broadcast_tx
                    .send(JobOutcomeAcquisitionResult::Succeeded(job_outcome))
                    .map_err(|e| {
                        RunnerRunError::InternalError(format!(
                            "Error while sending a job outcome: {}",
                            e
                        ))
                    })?;
                Ok(())
            }
            Err(e) => {
                outcome_broadcast_tx
                    .send(JobOutcomeAcquisitionResult::FailedUnexpectedly(
                        e.to_string(),
                    ))
                    .map_err(|e| {
                        RunnerRunError::InternalError(format!(
                            "Error while sending a job outcome: {}",
                            e
                        ))
                    })?;
                Ok(())
            }
        }
    }

    async fn run_execution_job(
        run_future: impl Future<
            Output = Result<ExecutionJobFinished<JobOutcomeType>, job::ExecutionJobError>,
        >,
        outcome_broadcast_tx: broadcast::Sender<JobOutcomeAcquisitionResult<JobOutcomeType>>,
    ) -> Result<ExecutionJobOutput, RunnerRunError> {
        let run_result = run_future.await;
        match run_result {
            Ok(ExecutionJobFinished::Succeeded(job_outcome, shell_output)) => {
                outcome_broadcast_tx
                    .send(JobOutcomeAcquisitionResult::Succeeded(job_outcome))
                    .map_err(|e| {
                        RunnerRunError::InternalError(format!(
                            "Error while sending a job outcome: {}",
                            e
                        ))
                    })?;
                Ok(ExecutionJobOutput::Succeeded(shell_output))
            }
            Ok(ExecutionJobFinished::PrecedingJobFailedExpectedly) => {
                outcome_broadcast_tx
                    .send(JobOutcomeAcquisitionResult::FailedExpectedly)
                    .map_err(|e| {
                        RunnerRunError::InternalError(format!(
                            "Error while sending a job outcome: {}",
                            e
                        ))
                    })?;
                Ok(ExecutionJobOutput::EarlyExit)
            }
            Ok(ExecutionJobFinished::FailedExpectedly((_, shell_output))) => {
                outcome_broadcast_tx
                    .send(JobOutcomeAcquisitionResult::FailedExpectedly)
                    .map_err(|e| {
                        RunnerRunError::InternalError(format!(
                            "Error while sending a job outcome: {}",
                            e
                        ))
                    })?;
                Ok(ExecutionJobOutput::FailedExpectedly(shell_output))
            }
            Err(e) => {
                outcome_broadcast_tx
                    .send(JobOutcomeAcquisitionResult::FailedUnexpectedly(
                        e.to_string(),
                    ))
                    .map_err(|e| {
                        RunnerRunError::InternalError(format!(
                            "Error while sending a job outcome: {}",
                            e
                        ))
                    })?;
                Ok(ExecutionJobOutput::EarlyExit)
            }
        }
    }
}
