use crate::model::editorials::EditorialRow;
use axum::async_trait;
use domain::{model::editorials::Editorial, repository::editorials::EditorialsRepository};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct EditorialRepositoryImpl {
    pool: MySqlPool,
}

impl EditorialRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EditorialsRepository for EditorialRepositoryImpl {
    async fn get_editorial(&self, id: i64) -> anyhow::Result<Option<Editorial>> {
        let editorial = sqlx::query_as::<_, EditorialRow>("SELECT * FROM editorials WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(editorial.map(|editorial| editorial.into()))
    }

    async fn create_editorial(&self, editorial: Editorial) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO editorials (problem_id, author_id, statement, created_at, updated_at, is_public) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(editorial.problem_id)
        .bind(editorial.author_id)
        .bind(editorial.statement)
        .bind(editorial.created_at)
        .bind(editorial.updated_at)
        .bind(editorial.is_public)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_editorial(&self, editorial: Editorial) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE editorials SET problem_id = ?, author_id = ?, statement = ?, created_at = ?, updated_at = ?, is_public = ? WHERE id = ?",
        )
        .bind(editorial.problem_id)
        .bind(editorial.author_id)
        .bind(editorial.statement)
        .bind(editorial.created_at)
        .bind(editorial.updated_at)
        .bind(editorial.is_public)
        .bind(editorial.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_editorial(&self, id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM editorials WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
