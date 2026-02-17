use crate::model::{
    editorial::{EditorialRow, EditorialSummaryRow},
    uuid::UuidRow,
};
use axum::async_trait;
use domain::{
    model::editorial::{
        CreateEditorial, Editorial, EditorialGetQuery, EditorialId, EditorialSummary,
        UpdateEditorial,
    },
    model::problem::ProblemId,
    model::user::UserDisplayId,
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
    async fn get_editorial(&self, id: EditorialId) -> anyhow::Result<Option<Editorial>> {
        let editorial = sqlx::query_as!(
            EditorialRow,
            r#"
            SELECT
                id AS "id: _",
                problem_id,
                author_id,
                statement,
                created_at AS "created_at: _",
                updated_at AS "updated_at: _",
                is_public AS "is_public: _",
                title
            FROM 
                editorials
            WHERE 
                id = ?
            "#,
            UuidRow(id.into())
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(editorial.map(|editorial| editorial.into()))
    }

    async fn get_editorials_by_problem_id(
        &self,
        query: EditorialGetQuery,
    ) -> anyhow::Result<Vec<EditorialSummary>> {
        let mut query_builder = QueryBuilder::new(
            r#"
            SELECT 
                * 
            FROM 
                editorials
            WHERE
            "#,
        );
        query_builder.push(" (is_public = TRUE");
        if let Some(user_id) = query.user_id {
            query_builder
                .push(" OR author_id = ")
                .push_bind(<UserDisplayId as Into<i64>>::into(user_id));
        }
        query_builder.push(")");

        query_builder
            .push(" AND problem_id = ")
            .push_bind(<ProblemId as Into<i64>>::into(query.problem_id));

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

    async fn create_editorial(&self, query: CreateEditorial) -> anyhow::Result<EditorialId> {
        let id = Uuid::now_v7();

        sqlx::query!(
            r#"
            INSERT INTO 
                editorials (
                    id, 
                    problem_id, 
                    author_id, 
                    statement, 
                    is_public, 
                    title
                ) 
            VALUES 
                (?, ?, ?, ?, ?, ?)
            "#,
            UuidRow(id.into()),
            <ProblemId as Into<i64>>::into(query.problem_id),
            <UserDisplayId as Into<i64>>::into(query.author_id),
            query.statement,
            query.is_public,
            query.title
        )
        .execute(&self.pool)
        .await?;

        Ok(id.into())
    }

    async fn update_editorial(&self, query: UpdateEditorial) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE
                editorials
            SET
                statement = ?,
                is_public = ?,
                title = ?
            WHERE 
                id = ?
            "#,
            query.statement,
            query.is_public,
            query.title,
            UuidRow(query.id.into())
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_editorial(&self, id: EditorialId) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM 
                editorials 
            WHERE 
                id = ?
            "#,
            UuidRow(id.into())
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
