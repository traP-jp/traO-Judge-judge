use axum::async_trait;
use uuid::Uuid;

use crate::model::icon::Icon;

#[async_trait]
pub trait IconRepository {
    async fn get_icon(&self, id: Uuid) -> anyhow::Result<Option<Icon>>;
    async fn create_icon(&self, icon: Icon) -> anyhow::Result<()>;
    async fn delete_icon(&self, id: Uuid) -> anyhow::Result<()>;
}
