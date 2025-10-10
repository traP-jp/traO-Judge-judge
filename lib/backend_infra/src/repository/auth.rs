use axum::async_trait;
use domain::{model::user::UserId, repository::auth::AuthRepository};
use sqlx::MySqlPool;

use crate::model::uuid::UuidRow;

#[derive(Clone)]
pub struct AuthRepositoryImpl {
    bcrypt_cost: u32,
    pool: MySqlPool,
}

impl AuthRepositoryImpl {
    pub fn new(bcrypt_cost: u32, pool: MySqlPool) -> Self {
        Self { bcrypt_cost, pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn count_authentication_methods(&self, id: UserId) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT (IF(password IS NOT NULL, 1, 0) + IF(github_oauth IS NOT NULL, 1, 0) + IF(google_oauth IS NOT NULL, 1, 0) + IF(traq_oauth IS NOT NULL, 1, 0)) AS authentication_count FROM user_authentications WHERE user_id = ?",
        )
        .bind(UuidRow(id.into()))
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    async fn save_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let hash = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query("INSERT INTO user_authentications (user_id, password) VALUES (?, ?)")
            .bind(UuidRow(id.into()))
            .bind(&hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let hash = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query("UPDATE user_authentications SET password = ? WHERE user_id = ?")
            .bind(&hash)
            .bind(UuidRow(id.into()))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn verify_user_password(&self, id: UserId, password: &str) -> anyhow::Result<bool> {
        let hash = sqlx::query_scalar::<_, String>(
            "SELECT password FROM user_authentications WHERE user_id = ?",
        )
        .bind(UuidRow(id.into()))
        .fetch_one(&self.pool)
        .await?;

        Ok(bcrypt::verify(password, &hash)?)
    }

    async fn get_google_oauth2_url(&self, oauth_action: &str) -> anyhow::Result<String> {
        let client_id = std::env::var("GOOGLE_OAUTH2_CLIENT_ID")?;
        let redirect_uri =
            std::env::var("FRONTEND_URL")? + &format!("/auth/google/{}/callback", oauth_action);
        let response_type = "code";
        let scope = "openid";
        let access_type = "online";
        match oauth_action {
            "login" | "signup" | "bind" => Ok(format!(
                "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope={}&access_type={}",
                client_id, redirect_uri, response_type, scope, access_type
            )),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn get_google_oauth_by_authorize_code(
        &self,
        code: &str,
        oauth_action: &str,
    ) -> anyhow::Result<String> {
        let client_id = std::env::var("GOOGLE_OAUTH2_CLIENT_ID")?;
        let client_secret = std::env::var("GOOGLE_OAUTH2_CLIENT_SECRET")?;
        let grant_type = "authorization_code";
        let redirect_uri =
            std::env::var("FRONTEND_URL")? + &format!("/auth/google/{}/callback", oauth_action);

        let url = format!(
            "https://oauth2.googleapis.com/token?client_id={}&client_secret={}&code={}&grant_type={}&redirect_uri={}",
            client_id, client_secret, code, grant_type, redirect_uri
        );

        let client = reqwest::Client::new();
        let response = client.post(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to exchange authorization code"));
        }

        let response_json = response.json::<serde_json::Value>().await?;

        let google_oauth = response_json
            .get("id_token")
            .and_then(|id_token| id_token.get("sub"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve Google OAuth"))?;

        match oauth_action {
            "login" | "signup" | "bind" => Ok(google_oauth.to_string()),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn save_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO user_authentications (user_id, google_oauth) VALUES (?, ?)")
            .bind(UuidRow(id.into()))
            .bind(google_oauth)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE user_authentications SET google_oauth = ? WHERE user_id = ?")
            .bind(google_oauth)
            .bind(UuidRow(id.into()))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn verify_user_google_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let google_oauth = sqlx::query_scalar::<_, Option<String>>(
            "SELECT google_oauth FROM user_authentications WHERE user_id = ?",
        )
        .bind(UuidRow(id.into()))
        .fetch_optional(&self.pool)
        .await?;

        Ok(google_oauth.is_some())
    }

    async fn delete_user_google_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        sqlx::query("UPDATE user_authentications SET google_oauth = NULL WHERE user_id = ?")
            .bind(UuidRow(id.into()))
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    async fn get_user_id_by_google_oauth(
        &self,
        google_oauth: &str,
    ) -> anyhow::Result<Option<UserId>> {
        let user_id = sqlx::query_scalar::<_, UuidRow>(
            "SELECT user_id FROM user_authentications WHERE google_oauth = ?",
        )
        .bind(google_oauth)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_id.map(|id| UserId(id.0)))
    }

    async fn get_github_oauth2_url(&self, oauth_action: &str) -> anyhow::Result<String> {
        let client_id = std::env::var("GITHUB_OAUTH2_CLIENT_ID")?;
        let redirect_uri =
            std::env::var("FRONTEND_URL")? + &format!("/auth/github/{}/callback", oauth_action);
        match oauth_action {
            "login" | "signup" | "bind" => Ok(format!(
                "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}",
                client_id, redirect_uri
            )),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn get_github_oauth_by_authorize_code(
        &self,
        code: &str,
        oauth_action: &str,
    ) -> anyhow::Result<String> {
        let client_id = std::env::var("GITHUB_OAUTH2_CLIENT_ID")?;
        let client_secret = std::env::var("GITHUB_OAUTH2_CLIENT_SECRET")?;
        let redirect_uri =
            std::env::var("FRONTEND_URL")? + &format!("/auth/github/{}/callback", oauth_action);

        let url = format!(
            "https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}&redirect_uri={}",
            client_id, client_secret, code, redirect_uri
        );

        let client = reqwest::Client::new();
        let response = client.post(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to exchange authorization code"));
        }

        let response_json = response.json::<serde_json::Value>().await?;

        let access_token = response_json
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve GitHub OAuth Access Token"))?;

        let response = client
            .get("https://api.github.com/user")
            .bearer_auth(access_token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to get GitHub User Info"));
        }

        let response_json = response.json::<serde_json::Value>().await?;

        let github_oauth = response_json
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow::anyhow!("Failed to retrieve GitHub OAuth"))?;

        match oauth_action {
            "login" | "signup" | "bind" => Ok(github_oauth.to_string()),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn save_user_github_oauth(&self, id: UserId, github_oauth: &str) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO user_authentications (user_id, github_oauth) VALUES (?, ?)")
            .bind(UuidRow(id.into()))
            .bind(github_oauth)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user_github_oauth(&self, id: UserId, github_oauth: &str) -> anyhow::Result<()> {
        sqlx::query("UPDATE user_authentications SET github_oauth = ? WHERE user_id = ?")
            .bind(github_oauth)
            .bind(UuidRow(id.into()))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn verify_user_github_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        let github_oauth = sqlx::query_scalar::<_, Option<String>>(
            "SELECT github_oauth FROM user_authentications WHERE user_id = ?",
        )
        .bind(UuidRow(id.into()))
        .fetch_optional(&self.pool)
        .await?;

        Ok(github_oauth.is_some())
    }

    async fn delete_user_github_oauth(&self, id: UserId) -> anyhow::Result<bool> {
        sqlx::query("UPDATE user_authentications SET github_oauth = NULL WHERE user_id = ?")
            .bind(UuidRow(id.into()))
            .execute(&self.pool)
            .await?;

        Ok(true)
    }

    async fn get_user_id_by_github_oauth(
        &self,
        github_oauth: &str,
    ) -> anyhow::Result<Option<UserId>> {
        let user_id = sqlx::query_scalar::<_, UuidRow>(
            "SELECT user_id FROM user_authentications WHERE github_oauth = ?",
        )
        .bind(github_oauth)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_id.map(|id| UserId(id.0)))
    }
}
