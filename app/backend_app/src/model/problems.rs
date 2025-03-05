use async_session::chrono;
use serde::{Deserialize, Serialize};
use usecase::model::problem::NormalProblemDto;

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
    pub judgecode_path: String,
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
            judgecode_path: problem.judgecode_path,
            created_at: problem.created_at,
            updated_at: problem.updated_at,
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
