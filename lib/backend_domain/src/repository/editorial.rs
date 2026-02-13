use axum::async_trait;

use crate::model::editorial::{
    CreateEditorial, Editorial, EditorialGetQuery, EditorialId, EditorialSummary, UpdateEditorial,
};
#[async_trait]
pub trait EditorialRepository {
    async fn get_editorial(&self, id: EditorialId) -> anyhow::Result<Option<Editorial>>;
    async fn get_editorials_by_problem_id(
        &self,
        query: EditorialGetQuery,
    ) -> anyhow::Result<Vec<EditorialSummary>>;
    async fn create_editorial(&self, editorial: CreateEditorial) -> anyhow::Result<EditorialId>;
    async fn update_editorial(&self, editorial: UpdateEditorial) -> anyhow::Result<()>;
    async fn delete_editorial(&self, id: EditorialId) -> anyhow::Result<()>;
}
