use crate::model::icon::IconId;
use sqlx::types::chrono;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserDisplayId(pub i64);

impl std::fmt::Display for UserDisplayId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<i64> for UserDisplayId {
    fn into(self) -> i64 {
        self.0
    }
}

impl From<i64> for UserDisplayId {
    fn from(id: i64) -> Self {
        Self(id)
    }
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

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    CommonUser,
    TrapUser,
    Admin,
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::CommonUser => 0,
            UserRole::TrapUser => 1,
            UserRole::Admin => 2,
        }
    }
}

impl UserRole {
    pub fn new(role: i32) -> anyhow::Result<Self> {
        match role {
            0 => Ok(UserRole::CommonUser),
            1 => Ok(UserRole::TrapUser),
            2 => Ok(UserRole::Admin),
            _ => anyhow::bail!("invalid role number"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub display_id: UserDisplayId,
    pub name: String,
    pub traq_id: Option<String>,
    pub github_id: Option<String>,
    pub icon_id: Option<IconId>,
    pub x_id: Option<String>,
    pub self_introduction: String,
    pub role: UserRole,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UpdateUser {
    pub user_name: String,
    pub icon_id: Option<IconId>,
    pub github_id: Option<String>,
    pub x_id: Option<String>,
    pub self_introduction: String,
}
