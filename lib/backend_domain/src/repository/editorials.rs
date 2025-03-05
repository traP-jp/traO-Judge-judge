use axum::async_trait;

use crate::model::editorials::Editorial;
#[async_trait]
pub trait EditorialsRepository {
    async fn get_editorial(&self, id: i64) -> anyhow::Result<Option<Editorial>>;
    async fn create_editorial(&self, editorial: Editorial) -> anyhow::Result<()>;
    async fn update_editorial(&self, editorial: Editorial) -> anyhow::Result<()>;
    async fn delete_editorial(&self, id: i64) -> anyhow::Result<()>;
}
