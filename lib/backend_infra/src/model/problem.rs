use sqlx::types::chrono;

use domain::model::problem::NormalProblem;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NormalProblemRow {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_kib: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<NormalProblemRow> for NormalProblem {
    fn from(val: NormalProblemRow) -> Self {
        NormalProblem {
            id: val.id,
            author_id: val.author_id,
            title: val.title,
            statement: val.statement,
            time_limit_ms: val.time_limit_ms,
            memory_limit_kib: val.memory_limit_kib,
            difficulty: val.difficulty,
            is_public: val.is_public,
            solved_count: val.solved_count,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
