use crate::model::user::{UserIdRow, UserRow};
use axum::async_trait;
use domain::{
    model::user::{UpdateUser, User, UserId},
    repository::user::UserRepository,
};
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: MySqlPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user_by_display_id(&self, display_id: i64) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE display_id = ?")
            .bind(display_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(|user| user.into()))
    }

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(|user| user.into()))
    }

    async fn create_user_by_email(&self, name: &str, email: &str) -> anyhow::Result<UserId> {
        let id = UserIdRow::new(Uuid::now_v7());

        sqlx::query("INSERT INTO users (id, name, email) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(UserId(id.0))
    }

    async fn update_user(&self, display_id: i64, body: UpdateUser) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET name = ?, icon_url = ?, x_link = ?, github_link = ?, self_introduction = ? WHERE display_id = ?")
            .bind(body.user_name)
            .bind(body.icon_url)
            .bind(body.x_link)
            .bind(body.github_link)
            .bind(body.self_introduction)
            .bind(display_id)
            .execute(&self.pool).await?;

        Ok(())
    }

    async fn is_exist_email(&self, email: &str) -> anyhow::Result<bool> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        Ok(count > 0)
    }
}
