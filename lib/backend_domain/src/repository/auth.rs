use axum::async_trait;

use crate::model::user::UserId;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait AuthRepository {
    async fn count_authentication_methods(&self, id: UserId) -> anyhow::Result<i64>;
    async fn save_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()>;
    async fn update_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()>;
    async fn verify_user_password(&self, id: UserId, password: &str) -> anyhow::Result<bool>;
    async fn get_google_oauth2_url(&self, oauth_action: &str) -> anyhow::Result<String>;
    async fn get_google_oauth_by_authorize_code(
        &self,
        code: &str,
        oauth_action: &str,
    ) -> anyhow::Result<String>;
    async fn save_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()>;
    async fn update_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()>;
    async fn verify_user_google_oauth(&self, id: UserId) -> anyhow::Result<bool>;
    async fn delete_user_google_oauth(&self, id: UserId) -> anyhow::Result<bool>;
    async fn get_user_id_by_google_oauth(
        &self,
        google_oauth: &str,
    ) -> anyhow::Result<Option<UserId>>;
    async fn get_github_oauth2_url(&self, oauth_action: &str) -> anyhow::Result<String>;
    async fn get_github_oauth_by_authorize_code(
        &self,
        code: &str,
        oauth_action: &str,
    ) -> anyhow::Result<String>;
    async fn save_user_github_oauth(&self, id: UserId, github_oauth: &str) -> anyhow::Result<()>;
    async fn update_user_github_oauth(&self, id: UserId, github_oauth: &str) -> anyhow::Result<()>;
    async fn verify_user_github_oauth(&self, id: UserId) -> anyhow::Result<bool>;
    async fn delete_user_github_oauth(&self, id: UserId) -> anyhow::Result<bool>;
    async fn get_user_id_by_github_oauth(
        &self,
        github_oauth: &str,
    ) -> anyhow::Result<Option<UserId>>;
}
