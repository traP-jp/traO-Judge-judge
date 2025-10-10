use crate::model::{user::UserRow, uuid::UuidRow};
use axum::async_trait;
use domain::{
    model::user::{UpdateUser, User, UserId, UserRole},
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

    async fn get_user_by_user_id(&self, id: UserId) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = ?")
            .bind(UuidRow(id.into()))
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(|user| user.into()))
    }

    async fn create_user_by_email(&self, name: &str, email: &str) -> anyhow::Result<UserId> {
        let id = UuidRow::new(Uuid::now_v7());

        sqlx::query("INSERT INTO users (id, name, email) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(UserId(id.0))
    }

    async fn create_user_without_email(&self, name: &str) -> anyhow::Result<UserId> {
        let id = UuidRow::new(Uuid::now_v7());

        sqlx::query("INSERT INTO users (id, name) VALUES (?, ?)")
            .bind(id)
            .bind(name)
            .execute(&self.pool)
            .await?;

        Ok(UserId(id.0))
    }

    async fn update_user(&self, display_id: i64, body: UpdateUser) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET name = ?, icon_id = ?, x_id = ?, self_introduction = ? WHERE display_id = ?")
            .bind(body.user_name)
            .bind(body.icon_id.map(UuidRow))
            .bind(body.x_id)
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

    async fn change_user_role(&self, user_id: UserId, role: UserRole) -> anyhow::Result<()> {
        sqlx::query("UPDATE users SET role = ? WHERE id = ?")
            .bind(role as i32)
            .bind(UuidRow(user_id.into()))
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
