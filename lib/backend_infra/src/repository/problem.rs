use crate::model::problem::NormalProblemRow;
use axum::async_trait;
use domain::{
    model::problem::{CreateNormalProblem, NormalProblem, UpdateNormalProblem},
    repository::problem::ProblemRepository,
};
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
            sqlx::query_as::<_, NormalProblemRow>("SELECT * FROM normal_problems WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(problem.map(|problem| problem.into()))
    }

    async fn update_problem(
        &self,
        id: i64,
        update_prblem: UpdateNormalProblem,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE normal_problems SET title = ?, is_public = ?, difficulty = ?, statement = ?, time_limit = ?, memory_limit = ? WHERE id = ?",
        )
        .bind(update_prblem.title)
        .bind(update_prblem.is_public)
        .bind(update_prblem.difficulty)
        .bind(update_prblem.statement)
        .bind(update_prblem.time_limit)
        .bind(update_prblem.memory_limit)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update problem: {}", e);
            e
        })?;

        Ok(())
    }

    async fn create_problem(&self, create_problem: CreateNormalProblem) -> anyhow::Result<i64> {
        let problem_id = sqlx::query(
            "INSERT INTO normal_problems (author_id, title, statement, time_limit, memory_limit, difficulty, judgecode_path) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(create_problem.author_id)
        .bind(create_problem.title)
        .bind(create_problem.statement)
        .bind(create_problem.time_limit)
        .bind(create_problem.memory_limit)
        .bind(create_problem.difficulty)
        .bind(create_problem.judgecode_path)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create problem: {}", e);
            e
        })?;

        Ok(problem_id.last_insert_id() as i64)
    }
}
