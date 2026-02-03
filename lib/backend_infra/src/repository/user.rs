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
        let user = sqlx::query_as!(
            UserRow,
            r#"
            SELECT 
                display_id,
                id AS "id: _",
                name,
                traq_id,
                icon_id AS "icon_id: _",
                github_id,
                x_id,
                self_introduction,
                role,
                created_at AS "created_at!: _",
                updated_at AS "updated_at!: _"
            FROM
                users 
            WHERE
                display_id = ?
            "#,
            display_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|user| user.into()))
    }

    async fn get_user_by_user_id(&self, id: UserId) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            UserRow,
            r#"
            SELECT 
                display_id,
                id AS "id: _",
                name,
                traq_id,
                icon_id AS "icon_id: _",
                github_id,
                x_id,
                self_introduction,
                role,
                created_at AS "created_at!: _",
                updated_at AS "updated_at!: _"
            FROM
                users 
            WHERE
                id = ?
            "#,
            UuidRow(id.into())
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|user| user.into()))
    }

    async fn create_user(&self, name: &str) -> anyhow::Result<UserId> {
        let id = UuidRow::new(Uuid::now_v7());

        sqlx::query!(
            r#"
            INSERT INTO 
                users (
                    id,
                    name
                ) 
            VALUES 
                (?, ?)
            "#,
            id,
            name
        )
        .execute(&self.pool)
        .await?;
        Ok(UserId(id.0))
    }

    async fn update_user(&self, display_id: i64, body: UpdateUser) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE
                users
            SET
                name = ?,
                icon_id = ?,
                github_id = ?,
                x_id = ?,
                self_introduction = ?
            WHERE
                display_id = ?
            "#,
            body.user_name,
            body.icon_id.map(UuidRow),
            body.github_id,
            body.x_id,
            body.self_introduction,
            display_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn change_user_role(&self, user_id: UserId, role: UserRole) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE
                users
            SET
                role = ?
            WHERE
                id = ?
            "#,
            role as i32,
            UuidRow(user_id.into())
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
