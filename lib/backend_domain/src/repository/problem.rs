use axum::async_trait;

use crate::model::problem::{CreateNormalProblem, NormalProblem, UpdateNormalProblem};

#[async_trait]
pub trait ProblemRepository {
    async fn get_problem(&self, id: i64) -> anyhow::Result<Option<NormalProblem>>;
    async fn update_problem(
        &self,
        id: i64,
        update_problem: UpdateNormalProblem,
    ) -> anyhow::Result<()>;
    async fn create_problem(&self, create_problem: CreateNormalProblem) -> anyhow::Result<i64>;
}
