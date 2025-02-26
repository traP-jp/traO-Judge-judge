use crate::model::problem::NormalProblemRow;
use axum::async_trait;
use domain::{
    model::problem::{CreateNormalProblems, NormalProblem, UpdateNormalProblems},
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
        update_prblem: UpdateNormalProblems,
    ) -> anyhow::Result<Option<NormalProblem>> {
        let problem = sqlx::query_as::<_, NormalProblemRow>(
            "UPDATE normal_problems SET title = ?, is_public = ?, difficulty = ?, statement = ?, time_limit = ?, memory_limit = ? WHERE id = ? RETURNING *",
        )
        .bind(update_prblem.title)
        .bind(update_prblem.is_public)
        .bind(update_prblem.difficulty)
        .bind(update_prblem.statement)
        .bind(update_prblem.time_limit)
        .bind(update_prblem.memory_limit)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(problem.map(|problem| problem.into()))
    }

    async fn create_problem(
        &self,
        create_problem: CreateNormalProblems,
    ) -> anyhow::Result<NormalProblem> {
        let problem = sqlx::query_as::<_, NormalProblemRow>(
            "INSERT INTO normal_problems (author_id, title, statement, time_limit, memory_limit, difficulty, judgecode_path) VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *",
        )
        .bind(create_problem.author_id)
        .bind(create_problem.title)
        .bind(create_problem.statement)
        .bind(create_problem.time_limit)
        .bind(create_problem.memory_limit)
        .bind(create_problem.difficulty)
        .bind(create_problem.judgecode_path)
        .fetch_one(&self.pool)
        .await?;

        Ok(problem.into())
    }
}
