use async_session::chrono;

#[derive(Debug, Clone)]
pub struct NormalProblem {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub judgecode_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UpdateNormalProblem {
    pub title: String,
    pub is_public: bool,
    pub difficulty: i32,
    pub statement: Option<String>,
    pub time_limit: i32,
    pub memory_limit: i32,
}

pub struct CreateNormalProblem {
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub judgecode_path: String,
}
