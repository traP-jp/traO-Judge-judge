use axum::async_trait;

use crate::model::user::UserId;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait AuthRepository {
    async fn save_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()>;
    async fn update_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()>;
    async fn verify_user_password(&self, id: UserId, password: &str) -> anyhow::Result<bool>;
}
