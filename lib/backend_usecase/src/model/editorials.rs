use sqlx::types::chrono;

use domain::model::editorials::Editorial;

#[derive(Debug, Clone)]
pub struct EditorialDto {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
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
            statement: value.statement,
            created_at: value.created_at,
            updated_at: value.updated_at,
            is_public: value.is_public,
        }
    }
}
