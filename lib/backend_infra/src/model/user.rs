use domain::model::user::{User, UserId, UserRole};
use sqlx::types::chrono;

use super::uuid::UuidRow;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRow {
    pub id: UuidRow,
    pub display_id: i64,
    pub name: String,
    pub traq_id: Option<String>,
    pub github_id: Option<String>,
    pub icon_id: Option<UuidRow>,
    pub x_id: Option<String>,
    pub self_introduction: String,
    pub role: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: UserId(row.id.0),
            display_id: row.display_id,
            name: row.name,
            traq_id: row.traq_id,
            github_id: row.github_id,
            icon_id: row.icon_id.map(|uuid_row| uuid_row.0),
            x_id: row.x_id,
            self_introduction: row.self_introduction,
            role: UserRole::new(row.role).unwrap(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
