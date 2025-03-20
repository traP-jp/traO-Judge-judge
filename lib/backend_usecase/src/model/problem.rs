use domain::model::problem::NormalProblem;
use sqlx::types::chrono;

pub struct UpdateNormalProblemData {
    pub title: Option<String>,
    pub statement: Option<String>,
    pub time_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub difficulty: Option<i32>,
    pub is_public: Option<bool>,
}

pub struct CreateNormalProblemData {
    // pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    // pub judgecode_path: String,
}

pub enum ProblemOrderByData {
    CreatedAtAsc,
    CreatedAtDesc,
    UpdatedAtAsc,
    UpdatedAtDesc,
    DifficultyAsc,
    DifficultyDesc,
}

pub struct ProblemGetQueryData {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: ProblemOrderByData,
    pub user_query: Option<i64>,
}

#[derive(Debug)]
pub struct NormalProblemDto {
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

impl From<NormalProblem> for NormalProblemDto {
    fn from(problem: NormalProblem) -> Self {
        NormalProblemDto {
            id: problem.id,
            author_id: problem.author_id,
            title: problem.title,
            statement: problem.statement,
            time_limit: problem.time_limit,
            memory_limit: problem.memory_limit,
            difficulty: problem.difficulty,
            is_public: problem.is_public,
            solved_count: problem.solved_count,
            judgecode_path: problem.judgecode_path,
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        }
    }
}

pub struct NormalProblemSummaryDto {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub judgecode_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct NormalProblemsDto {
    pub total: i64,
    pub problems: Vec<NormalProblemSummaryDto>,
}

impl From<NormalProblem> for NormalProblemSummaryDto {
    fn from(problem: NormalProblem) -> Self {
        NormalProblemSummaryDto {
            id: problem.id,
            author_id: problem.author_id,
            title: problem.title,
            time_limit: problem.time_limit,
            memory_limit: problem.memory_limit,
            difficulty: problem.difficulty,
            is_public: problem.is_public,
            solved_count: problem.solved_count,
            judgecode_path: problem.judgecode_path,
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        }
    }
}
