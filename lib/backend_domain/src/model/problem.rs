use crate::model::user::UserDisplayId;
use anyhow::Context;
use async_session::chrono;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProblemId(i64);

impl FromStr for ProblemId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .parse::<i64>()
            .context("failed to parse ProblemId from str")?;
        Ok(ProblemId(id))
    }
}

impl std::fmt::Display for ProblemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<i64> for ProblemId {
    fn into(self) -> i64 {
        self.0
    }
}

impl From<i64> for ProblemId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}
#[derive(Debug, Clone)]
pub struct NormalProblem {
    pub id: ProblemId,
    pub author_id: UserDisplayId,
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

pub struct UpdateNormalProblem {
    pub title: String,
    pub is_public: bool,
    pub difficulty: i32,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_kib: i32,
}

pub struct CreateNormalProblem {
    pub author_id: UserDisplayId,
    pub title: String,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_kib: i32,
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
    pub user_id: Option<UserDisplayId>,
    pub limit: i64,
    pub offset: i64,
    pub order_by: ProblemOrderBy,
    pub user_name: Option<String>,
    pub user_query: Option<UserDisplayId>,
}
