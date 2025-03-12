use super::identifiers::ResourceId;
use std::process::Output;

#[axum::async_trait]
pub trait JobApi<ReservationToken: Send + Sync, OutcomeToken: Clone + Send + Sync>:
    Clone + Send + Sync
{
    async fn reserve_execution(
        &self,
        count: usize,
    ) -> Result<Vec<ReservationToken>, ReservationError>;

    async fn place_file(&self, file_conf: FileConf) -> Result<OutcomeToken, FilePlacementError>;

    async fn execute(
        &self,
        reservation: ReservationToken,
        dependencies: Vec<Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, Output), ExecutionError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReservationError {
    #[error("Failed to reserve execution with error: {0}")]
    ReserveFailed(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum FilePlacementError {
    #[error("Failed to place file with error: {0}")]
    PlaceFailed(String),
    #[error("Invalid resource ID: {0}")]
    InvalidResourceId(ResourceId),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExecutionError {
    #[error("Internal error while executing a job: {0}")]
    InternalError(String),
    #[error("Judge process failed with error: {0}")]
    JudgeFailed(String),
}

#[derive(Debug, Clone)]
pub enum FileConf {
    EmptyDirectory,
    Text(ResourceId),
    RuntimeText(String),
}

#[derive(Debug, Clone)]
pub struct Dependency<OutcomeToken> {
    pub envvar: String,
    pub outcome: OutcomeToken,
}
