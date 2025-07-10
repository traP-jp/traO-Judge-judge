use sqlx::types::chrono;

use domain::model::editorial::{Editorial, EditorialSummary};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EditorialRow {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
    pub title: String,
}

impl From<EditorialRow> for Editorial {
    fn from(val: EditorialRow) -> Self {
        Editorial {
            id: val.id,
            problem_id: val.problem_id,
            author_id: val.author_id,
            title: val.title,
            statement: val.statement,
            created_at: val.created_at,
            updated_at: val.updated_at,
            is_public: val.is_public,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EditorialSummaryRow {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
    pub title: String,
}

impl From<EditorialSummaryRow> for EditorialSummary {
    fn from(val: EditorialSummaryRow) -> Self {
        EditorialSummary {
            id: val.id,
            problem_id: val.problem_id,
            author_id: val.author_id,
            title: val.title,
            created_at: val.created_at,
            updated_at: val.updated_at,
            is_public: val.is_public,
        }
    }
}
