use axum::async_trait;
use uuid::Uuid;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait ResourceIdCounterRepository {
    async fn get_deletable_resource_ids(&self, limit: usize) -> anyhow::Result<Vec<Uuid>>;
    async fn delete_resource_ids(&self, ids: Vec<Uuid>) -> anyhow::Result<()>;
    async fn update_timestamp_ids(&self, ids: Vec<Uuid>) -> anyhow::Result<()>;
}
