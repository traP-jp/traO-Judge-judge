use crate::model::testcase::TestcaseSummaryDto;
use domain::model::problem::NormalProblem;
use sqlx::types::chrono;
use validator::Validate;

#[derive(Validate)]
pub struct UpdateNormalProblemData {
    pub title: Option<String>,
    pub statement: Option<String>,
    #[validate(range(min = 1, max = 65535))]
    pub time_limit: Option<i32>,
    #[validate(range(min = 1, max = 65535))]
    pub memory_limit: Option<i32>,
    #[validate(range(min = 1, max = 10))]
    pub difficulty: Option<i32>,
    pub is_public: Option<bool>,
}

#[derive(Validate)]
pub struct CreateNormalProblemData {
    // pub author_id: i64,
    pub title: String,
    pub statement: String,
    #[validate(range(min = 1, max = 65535))]
    pub time_limit: i32,
    #[validate(range(min = 1, max = 65535))]
    pub memory_limit: i32,
    #[validate(range(min = 1, max = 10))]
    pub difficulty: i32,
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
    pub user_name: Option<String>,
    pub user_query: Option<String>,
}

pub struct NormalProblemDto {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub testcases: Vec<TestcaseSummaryDto>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<NormalProblem> for NormalProblemDto {
    fn from(problem: NormalProblem) -> Self {
        NormalProblemDto {
            id: problem.id.to_string(),
            author_id: problem.author_id.to_string(),
            title: problem.title,
            statement: problem.statement,
            time_limit: problem.time_limit,
            memory_limit: problem.memory_limit,
            difficulty: problem.difficulty,
            is_public: problem.is_public,
            solved_count: problem.solved_count,
            testcases: vec![],
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        }
    }
}

pub struct NormalProblemSummaryDto {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
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
            id: problem.id.to_string(),
            author_id: problem.author_id.to_string(),
            title: problem.title,
            time_limit: problem.time_limit,
            memory_limit: problem.memory_limit,
            difficulty: problem.difficulty,
            is_public: problem.is_public,
            solved_count: problem.solved_count,
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        }
    }
}
