use std::fmt;

use serde::Serialize;
use sqlx::FromRow;
use sqlx::Type;
use sqlx::{types::chrono, Decode, Encode, MySql};
use uuid::Uuid;

use super::Repository;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, sqlx::Type, Serialize)]
#[repr(i32)]
pub enum UserRole {
    #[serde(rename = "commonUser")]
    common_user = 0,
    #[serde(rename = "traPUser")]
    traP_user = 1,
    admin = 2,
}

#[derive(Debug, FromRow, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UserId(Uuid);

#[derive(Debug, FromRow, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub display_id: i64,
    pub name: String,
    pub traq_id: Option<String>,
    pub github_id: Option<String>,
    pub icon_url: Option<String>,
    pub x_link: Option<String>,
    pub github_link: Option<String>,
    pub self_introduction: String,
    pub role: UserRole,
    // todo: add more fields
    //pub post_problems:
    //pub submut_problems:
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UpdateUser {
    pub user_name: String,
    pub icon_url: Option<String>,
    pub x_link: Option<String>,
    pub github_link: Option<String>,
    pub self_introduction: String,
}

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Decode<'a, MySql> for UserId {
    fn decode(
        value: <MySql as sqlx::database::HasValueRef<'a>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <Uuid as Decode<'a, MySql>>::decode(value).map(UserId)
    }
}

impl<'a> Encode<'a, MySql> for UserId {
    fn encode_by_ref(
        &self,
        buf: &mut <MySql as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.encode_by_ref(buf)
    }

    fn encode(
        self,
        buf: &mut <MySql as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        self.0.encode(buf)
    }
}

impl Type<MySql> for UserId {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <Uuid as Type<MySql>>::type_info()
    }
    fn compatible(ty: &<MySql as sqlx::Database>::TypeInfo) -> bool {
        <Uuid as Type<MySql>>::compatible(ty)
    }
}

impl Repository {
    pub async fn get_user_by_display_id(&self, display_id: i64) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE display_id = ?")
            .bind(display_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user_by_email(&self, name: &str, email: &str) -> anyhow::Result<UserId> {
        let id = UserId::new(Uuid::now_v7());

        sqlx::query("INSERT INTO users (id, name, email) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(email)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    pub async fn update_user(&self, display_id: i64, body: UpdateUser) -> anyhow::Result<()> {
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

    pub async fn is_exist_email(&self, email: &str) -> anyhow::Result<bool> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.is_some())
    }
}
