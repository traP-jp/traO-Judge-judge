use crate::model::problem::NormalProblemPow;
use axum::async_trait;
use domain::{model::problem::NormalProblem, repository::problem::ProblemRepository};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct ProblemRepositoryImpl {
    pool: MySqlPool,
}

impl ProblemRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProblemRepository for ProblemRepositoryImpl {
    async fn get_problem(&self, id: i64) -> anyhow::Result<Option<NormalProblem>> {
        let problem =
            sqlx::query_as::<_, NormalProblemPow>("SELECT * FROM normal_problems WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(problem.map(|problem| problem.into()))
    }
}
