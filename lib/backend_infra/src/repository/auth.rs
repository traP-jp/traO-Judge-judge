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
    async fn save_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let hash = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query("INSERT INTO users_passwords (user_id, password) VALUES (?, ?)")
            .bind(UuidRow(id.into()))
            .bind(&hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let hash = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query("UPDATE users_passwords SET password = ? WHERE user_id = ?")
            .bind(&hash)
            .bind(UuidRow(id.into()))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn verify_user_password(&self, id: UserId, password: &str) -> anyhow::Result<bool> {
        let hash = sqlx::query_scalar::<_, String>(
            "SELECT password FROM users_passwords WHERE user_id = ?",
        )
        .bind(UuidRow(id.into()))
        .fetch_one(&self.pool)
        .await?;

        Ok(bcrypt::verify(password, &hash)?)
    }
}
