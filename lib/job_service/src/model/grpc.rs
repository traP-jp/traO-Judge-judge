use judge_core::model::job;
use uuid::Uuid;

use crate::jobapi::OutcomeToken;

#[axum::async_trait]
pub trait GrpcClient {
    async fn execute(
        &mut self,
        outcome_id_for_res: Uuid,
        dependency: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError>;
}
