use crate::model::testcase::TestcaseSammary;
use async_session::chrono;
use serde::{Deserialize, Serialize};
use usecase::model::problem::{NormalProblemDto, NormalProblemSummaryDto, NormalProblemsDto};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemResponse {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub testcases: Vec<TestcaseSammary>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<NormalProblemDto> for ProblemResponse {
    fn from(problem: NormalProblemDto) -> Self {
        ProblemResponse {
            id: problem.id,
            author_id: problem.author_id,
            title: problem.title,
            statement: problem.statement,
            time_limit: problem.time_limit,
            memory_limit: problem.memory_limit,
            difficulty: problem.difficulty,
            is_public: problem.is_public,
            solved_count: problem.solved_count,
            testcases: problem.testcases.into_iter().map(|x| x.into()).collect(),
            created_at: problem.created_at,
            updated_at: problem.updated_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemSummaryResponse {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
    pub is_public: bool,
    pub solved_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<NormalProblemSummaryDto> for ProblemSummaryResponse {
    fn from(problem: NormalProblemSummaryDto) -> Self {
        ProblemSummaryResponse {
            id: problem.id,
            author_id: problem.author_id,
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemResponses {
    pub total: i64,
    pub problems: Vec<ProblemSummaryResponse>,
}

impl From<NormalProblemsDto> for ProblemResponses {
    fn from(problems: NormalProblemsDto) -> Self {
        ProblemResponses {
            total: problems.total,
            problems: problems.problems.into_iter().map(|p| p.into()).collect(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNormalProblem {
    pub title: Option<String>,
    pub statement: Option<String>,
    pub time_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub difficulty: Option<i32>,
    pub is_public: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNormalProblem {
    pub title: String,
    pub statement: String,
    pub time_limit: i32,
    pub memory_limit: i32,
    pub difficulty: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProblemOrderBy {
    CreatedAtAsc,
    CreatedAtDesc,
    UpdatedAtAsc,
    UpdatedAtDesc,
    DifficultyAsc,
    DifficultyDesc,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemGetQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Option<ProblemOrderBy>,
    pub user_id: Option<i64>,
}
