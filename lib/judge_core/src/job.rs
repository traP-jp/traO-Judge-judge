use crate::identifiers::ResourceId;
use futures::future::Future;
use std::process::Output;
use tokio::sync::broadcast;

/// JobAPI is a set of shell environment and cache of outcome files of previous jobs.
///
/// Instances must be initialized once per submission.
pub trait JobApi<JobOutcome: Clone>: Clone {
    /// Greater the priority is, sooner the job will be executed.
    ///
    /// Files created by this job will be deleted immediately after all returned JobOutcome are dropped.
    ///
    /// Outer future only creates a kind of reservasion for shell environment and returns inner future synchronously.
    fn run_future(
        &self,
        job_conf: ExecutionJob<JobOutcome>,
    ) -> impl Future<
        Output = Result<
            impl Future<Output = Result<ExecutionJobFinished<JobOutcome>, ExecutionJobError>>,
            ExecutionJobPreparationError,
        >,
    >;

    fn place_file(
        &self,
        job_conf: FilePlacementJob,
    ) -> impl Future<Output = Result<JobOutcome, FilePlacementJobError>>;
}

#[derive(Debug, Clone)]
pub enum ExecutionJobFinished<JobOutcome: Clone> {
    /// Job finished successfully.
    Succeeded(JobOutcome, Output),
    /// Job failed expectedly.
    FailedExpectedly((JobOutcome, Output)),
    /// Preceding job failed expectedly.
    PrecedingJobFailedExpectedly,
}

#[derive(Debug, Clone)]
pub enum JobOutcomeAcquisitionResult<JobOutcome: Clone> {
    /// Received JobOutcome successfully.
    Succeeded(JobOutcome),
    /// Failed to receive JobOutcome expectedly.
    FailedExpectedly,
    /// Failed to receive JobOutcome unexpectedly.
    FailedUnexpectedly(String),
}

pub struct JobOutcomeLink<JobOutcome: Clone> {
    pub job_outcome_rx: broadcast::Receiver<JobOutcomeAcquisitionResult<JobOutcome>>,
    pub envvar_name: String,
}

pub struct ExecutionJob<JobOutcome: Clone> {
    pub script: String,
    pub depends_on_with_names: Vec<JobOutcomeLink<JobOutcome>>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExecutionJobPreparationError {
    #[error("Internal error while preparing a job: {0}")]
    InternalError(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExecutionJobError {
    #[error("Internal error while running a job: {0}")]
    InternalError(String),
    #[error("Preceding job failed unexpectedly: {0}")]
    PrecedingJobFailed(String),
}

pub enum FilePlacementJob {
    PlaceEmptyDirectory,
    /// Content of the text file
    PlaceRuntimeTextFile(String),
    /// Global project-wide unique identifier
    PlaceTextFile(ResourceId),
}

#[derive(Debug, thiserror::Error)]
pub enum FilePlacementJobError {
    #[error("Invalid resource id: {0}")]
    InvalidResourceId(ResourceId),
    #[error("Internal error while placing a file: {0}")]
    InternalError(String),
}
