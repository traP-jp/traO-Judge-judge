use axum::async_trait;

use crate::model::problem::NormalProblem;

#[async_trait]
pub trait ProblemRepository {
    async fn get_problem(&self, id: i64) -> anyhow::Result<Option<NormalProblem>>;
}
