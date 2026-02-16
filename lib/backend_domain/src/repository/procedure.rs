use crate::model::problem::ProblemId;
use axum::async_trait;
use judge_core::model::procedure::registered::Procedure;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait ProcedureRepository {
    async fn create_procedure(
        &self,
        problem_id: ProblemId,
        procedure: Procedure,
    ) -> anyhow::Result<()>;
    async fn update_procedure(
        &self,
        problem_id: ProblemId,
        procedure: Procedure,
    ) -> anyhow::Result<()>;
    async fn get_procedure(&self, problem_id: ProblemId) -> anyhow::Result<Option<Procedure>>;
    async fn delete_procedure(&self, problem_id: ProblemId) -> anyhow::Result<()>;
}
