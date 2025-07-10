use sqlx::types::chrono;

use domain::model::editorial::{Editorial, EditorialSummary};

#[derive(Debug, Clone)]
pub struct CreateEditorialData {
    pub statement: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateEditorialData {
    pub statement: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct EditorialDto {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
}

impl From<Editorial> for EditorialDto {
    fn from(value: Editorial) -> Self {
        EditorialDto {
            id: value.id,
            problem_id: value.problem_id,
            author_id: value.author_id,
            title: "解説",
            statement: value.statement,
            created_at: value.created_at,
            updated_at: value.updated_at,
            is_public: value.is_public,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditorialSummaryDto {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
}

impl From<EditorialSummary> for EditorialSummaryDto {
    fn from(value: EditorialSummary) -> Self {
        EditorialSummaryDto {
            id: value.id,
            problem_id: value.problem_id,
            author_id: value.author_id,
            title: "解説",
            created_at: value.created_at,
            updated_at: value.updated_at,
            is_public: value.is_public,
        }
    }
}
