use axum::async_trait;

use crate::model::session::SessionUser;
use crate::model::user::{User, UserDisplayId, UserId};

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait SessionRepository {
    async fn create_session(&self, user: User) -> anyhow::Result<String>;
    async fn delete_session(&self, session_id: &str) -> anyhow::Result<Option<()>>;
    async fn get_session_user(&self, session_id: &str) -> anyhow::Result<Option<SessionUser>>;
}
