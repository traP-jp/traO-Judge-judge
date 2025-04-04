use axum::async_trait;

use crate::model::problem::{
    CreateNormalProblem, NormalProblem, ProblemGetQuery, UpdateNormalProblem,
};

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait ProblemRepository {
    async fn get_problem(&self, id: i64) -> anyhow::Result<Option<NormalProblem>>;
    async fn get_problems_by_query(
        &self,
        query: ProblemGetQuery,
    ) -> anyhow::Result<Vec<NormalProblem>>;
    async fn get_problems_by_query_count(&self, query: ProblemGetQuery) -> anyhow::Result<i64>;
    async fn update_problem(
        &self,
        id: i64,
        update_problem: UpdateNormalProblem,
    ) -> anyhow::Result<()>;
    async fn create_problem(&self, create_problem: CreateNormalProblem) -> anyhow::Result<i64>;
}
