use axum::async_trait;

use crate::model::icon::{CreateIcon, Icon, IconId};

#[async_trait]
pub trait IconRepository {
    async fn get_icon(&self, id: IconId) -> anyhow::Result<Option<Icon>>;
    async fn create_icon(&self, create_icon: CreateIcon) -> anyhow::Result<IconId>;
    async fn delete_icon(&self, id: IconId) -> anyhow::Result<()>;
}
