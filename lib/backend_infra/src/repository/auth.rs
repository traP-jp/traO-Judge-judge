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
        match oauth_action {
            "login" => Ok("https://example.com/google_oauth2_login".to_string()),
            "signup" => Ok("https://example.com/google_oauth2_signup".to_string()),
            "bind" => Ok("https://example.com/google_oauth2_bind".to_string()),
            _ => Err(anyhow::anyhow!("Invalid OAuth action")),
        }
    }

    async fn get_google_oauth_by_authorize_code(&self, code: &str, oauth_action: &str) -> anyhow::Result<String> {
        match oauth_action {
            "login" => Ok(format!("https://example.com/google_oauth2_login/{}", code)),
            "signup" => Ok(format!("https://example.com/google_oauth2_signup/{}", code)),
            "bind" => Ok(format!("https://example.com/google_oauth2_bind/{}", code)),
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

    // async fn update_user_google_oauth(&self, id: UserId, google_oauth: &str) -> anyhow::Result<()> {
    //     sqlx::query("UPDATE user_authentications SET google_oauth = ? WHERE user_id = ?")
    //         .bind(google_oauth)
    //         .bind(UuidRow(id.into()))
    //         .execute(&self.pool)
    //         .await?;

    //     Ok(())
    // }

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

    async fn get_user_id_by_google_oauth(&self, google_oauth: &str) -> anyhow::Result<Option<UserId>> {
        let user_id = sqlx::query_scalar::<_, UuidRow>(
            "SELECT user_id FROM user_authentications WHERE google_oauth = ?",
        )
        .bind(google_oauth)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_id.map(|id| UserId(id.0)))
    }
}
