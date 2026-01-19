use crate::model::{
    editorial::{EditorialRow, EditorialSummaryRow},
    uuid::UuidRow,
};
use axum::async_trait;
use domain::{
    model::editorial::{
        CreateEditorial, Editorial, EditorialGetQuery, EditorialSummary, UpdateEditorial,
    },
    repository::editorial::EditorialRepository,
};
use sqlx::{MySqlPool, QueryBuilder};
use uuid::Uuid;

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
impl EditorialRepository for EditorialRepositoryImpl {
    async fn get_editorial(&self, id: Uuid) -> anyhow::Result<Option<Editorial>> {
        let editorial = sqlx::query_as::<_, EditorialRow>("SELECT * FROM editorials WHERE id = ?")
            .bind(UuidRow(id))
            .fetch_optional(&self.pool)
            .await?;

        Ok(editorial.map(|editorial| editorial.into()))
    }

    async fn get_editorials_by_problem_id(
        &self,
        query: EditorialGetQuery,
    ) -> anyhow::Result<Vec<EditorialSummary>> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM editorials WHERE");
        query_builder.push(" (is_public = TRUE");
        if let Some(user_id) = query.user_id {
            query_builder.push(" OR author_id = ").push_bind(user_id);
        }
        query_builder.push(")");

        query_builder
            .push(" AND problem_id = ")
            .push_bind(query.problem_id);

        query_builder.push(" ORDER BY created_at DESC");
        query_builder.push(" LIMIT ").push_bind(query.limit);
        query_builder.push(" OFFSET ").push_bind(query.offset);

        let editorials = query_builder
            .build_query_as::<EditorialSummaryRow>()
            .fetch_all(&self.pool)
            .await?;

        Ok(editorials
            .into_iter()
            .map(|editorial| editorial.into())
            .collect())
    }

    async fn create_editorial(&self, query: CreateEditorial) -> anyhow::Result<Uuid> {
        let id = Uuid::now_v7();

        sqlx::query(
            "INSERT INTO editorials (id, problem_id, author_id, statement, is_public, title) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(UuidRow(id))
        .bind(query.problem_id)
        .bind(query.author_id)
        .bind(query.statement)
        .bind(query.is_public)
        .bind(query.title)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn update_editorial(&self, query: UpdateEditorial) -> anyhow::Result<()> {
        sqlx::query("UPDATE editorials SET statement = ?, is_public = ? , title = ? WHERE id = ?")
            .bind(query.statement)
            .bind(query.is_public)
            .bind(query.title)
            .bind(UuidRow(query.id))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_editorial(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM editorials WHERE id = ?")
            .bind(UuidRow(id))
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
