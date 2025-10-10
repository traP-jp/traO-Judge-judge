use async_session::chrono;

#[derive(Debug, Clone)]
pub struct NormalProblem {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_mib: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct UpdateNormalProblem {
    pub title: String,
    pub is_public: bool,
    pub difficulty: i32,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_mib: i32,
}

pub struct CreateNormalProblem {
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_mib: i32,
    pub difficulty: i32,
}

#[derive(Clone)]
pub enum ProblemOrderBy {
    CreatedAtAsc,
    CreatedAtDesc,
    UpdatedAtAsc,
    UpdatedAtDesc,
    DifficultyAsc,
    DifficultyDesc,
}

#[derive(Clone)]
pub struct ProblemGetQuery {
    pub user_id: Option<i64>,
    pub limit: i64,
    pub offset: i64,
    pub order_by: ProblemOrderBy,
    pub user_name: Option<String>,
    pub user_query: Option<i64>,
}
