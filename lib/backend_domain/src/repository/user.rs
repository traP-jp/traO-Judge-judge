use axum::async_trait;

use crate::model::user::{UpdateUser, User, UserId};

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait UserRepository {
    async fn get_user_by_display_id(&self, display_id: i64) -> anyhow::Result<Option<User>>;
    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn create_user_by_email(&self, name: &str, email: &str) -> anyhow::Result<UserId>;
    async fn update_user(&self, display_id: i64, body: UpdateUser) -> anyhow::Result<()>;
    async fn is_exist_email(&self, email: &str) -> anyhow::Result<bool>;
}
