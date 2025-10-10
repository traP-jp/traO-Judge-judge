use crate::model::problem::NormalProblemRow;
use axum::async_trait;
use domain::{
    model::problem::{
        CreateNormalProblem, NormalProblem, ProblemGetQuery, ProblemOrderBy, UpdateNormalProblem,
    },
    repository::problem::ProblemRepository,
};
use sqlx::{MySqlPool, QueryBuilder};

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

    async fn get_problems_by_query(
        &self,
        query: ProblemGetQuery,
    ) -> anyhow::Result<Vec<NormalProblem>> {
        let mut query_builder = QueryBuilder::new(
            "SELECT normal_problems.*, users.name, users.display_id FROM normal_problems LEFT JOIN users ON normal_problems.author_id = users.display_id WHERE",
        );

        query_builder.push(" (normal_problems.is_public = TRUE");
        if let Some(user_id) = query.user_id {
            query_builder
                .push(" OR normal_problems.author_id = ")
                .push_bind(user_id);
        }
        query_builder.push(")");

        if let Some(user_query) = query.user_query {
            query_builder
                .push(" AND normal_problems.author_id = ")
                .push_bind(user_query);
        }

        if let Some(user_name) = query.user_name {
            query_builder
                .push(" AND users.name = ")
                .push_bind(user_name);
        }

        query_builder.push(" ORDER BY ");

        match query.order_by {
            ProblemOrderBy::CreatedAtAsc => {
                query_builder.push("normal_problems.created_at ASC");
            }
            ProblemOrderBy::CreatedAtDesc => {
                query_builder.push("normal_problems.created_at DESC");
            }
            ProblemOrderBy::UpdatedAtAsc => {
                query_builder.push("normal_problems.updated_at ASC");
            }
            ProblemOrderBy::UpdatedAtDesc => {
                query_builder.push("normal_problems.updated_at DESC");
            }
            ProblemOrderBy::DifficultyAsc => {
                query_builder.push("normal_problems.difficulty ASC");
            }
            ProblemOrderBy::DifficultyDesc => {
                query_builder.push("normal_problems.difficulty DESC");
            }
        }

        query_builder.push(" LIMIT ").push_bind(query.limit);
        query_builder.push(" OFFSET ").push_bind(query.offset);

        let problems = query_builder
            .build_query_as::<NormalProblemRow>()
            .fetch_all(&self.pool)
            .await?;

        Ok(problems.into_iter().map(|problem| problem.into()).collect())
    }

    async fn get_problems_by_query_count(&self, query: ProblemGetQuery) -> anyhow::Result<i64> {
        let mut query_builder = QueryBuilder::new(
            "SELECT COUNT(1) FROM normal_problems LEFT JOIN users ON normal_problems.author_id = users.display_id WHERE",
        );

        query_builder.push(" (normal_problems.is_public = TRUE");
        if let Some(user_id) = query.user_id {
            query_builder
                .push(" OR normal_problems.author_id = ")
                .push_bind(user_id);
        }
        query_builder.push(")");

        if let Some(user_query) = query.user_query {
            query_builder
                .push(" AND normal_problems.author_id = ")
                .push_bind(user_query);
        }

        if let Some(user_name) = query.user_name {
            query_builder
                .push(" AND users.name = ")
                .push_bind(user_name);
        }

        let count = query_builder
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
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
            "INSERT INTO normal_problems (author_id, title, statement, time_limit, memory_limit, difficulty) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(create_problem.author_id)
        .bind(create_problem.title)
        .bind(create_problem.statement)
        .bind(create_problem.time_limit)
        .bind(create_problem.memory_limit)
        .bind(create_problem.difficulty)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create problem: {}", e);
            e
        })?;

        Ok(problem_id.last_insert_id() as i64)
    }

    async fn delete_problem(&self, id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM normal_problems WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
