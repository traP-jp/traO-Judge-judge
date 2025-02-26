use axum::async_trait;

use crate::model::problem::{CreateNormalProblems, NormalProblem, UpdateNormalProblems};

#[async_trait]
pub trait ProblemRepository {
    async fn get_problem(&self, id: i64) -> anyhow::Result<Option<NormalProblem>>;
    async fn update_problem(
        &self,
        id: i64,
        update_problem: UpdateNormalProblems,
    ) -> anyhow::Result<Option<NormalProblem>>;
    async fn create_problem(
        &self,
        create_problem: CreateNormalProblems,
    ) -> anyhow::Result<NormalProblem>;
}
