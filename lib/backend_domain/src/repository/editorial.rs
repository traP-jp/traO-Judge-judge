use axum::async_trait;

use crate::model::editorial::{
    CreateEditorial, Editorial, EditorialGetQuery, EditorialSummary, UpdateEditorial,
};
#[async_trait]
pub trait EditorialRepository {
    async fn get_editorial(&self, id: i64) -> anyhow::Result<Option<Editorial>>;
    async fn get_editorials_by_problem_id(
        &self,
        query: EditorialGetQuery,
    ) -> anyhow::Result<Vec<EditorialSummary>>;
    async fn create_editorial(&self, editorial: CreateEditorial) -> anyhow::Result<i64>;
    async fn update_editorial(&self, editorial: UpdateEditorial) -> anyhow::Result<()>;
    async fn delete_editorial(&self, id: i64) -> anyhow::Result<()>;
}
