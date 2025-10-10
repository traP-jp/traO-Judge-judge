use sqlx::types::chrono;

use domain::model::submission::{JudgeResult, Submission};

use crate::model::uuid::UuidRow;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SubmissionRow {
    pub id: UuidRow,
    pub problem_id: i64,
    pub problem_title: String,
    pub user_id: i64,
    pub user_name: String,
    pub language_id: i32,
    pub source: String,
    pub judge_status: String,
    pub total_score: i64,
    pub max_time_ms: i32,
    pub max_memory_mib: i32,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

impl From<SubmissionRow> for Submission {
    fn from(val: SubmissionRow) -> Self {
        Submission {
            id: val.id.0,
            problem_id: val.problem_id,
            problem_title: val.problem_title,
            user_id: val.user_id,
            user_name: val.user_name,
            language_id: val.language_id,
            source: val.source,
            overall_judge_status: val.judge_status,
            total_score: val.total_score,
            max_time_ms: val.max_time_ms,
            max_memory_mib: val.max_memory_mib,
            submitted_at: val.submitted_at,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct JudgeResultRow {
    pub submission_id: UuidRow,
    pub testcase_id: UuidRow,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time_ms: i32,
    pub memory_mib: i32,
}

impl From<JudgeResultRow> for JudgeResult {
    fn from(val: JudgeResultRow) -> Self {
        JudgeResult {
            testcase_id: val.testcase_id.0,
            testcase_name: val.testcase_name,
            judge_status: val.judge_status,
            score: val.score,
            time_ms: val.time_ms,
            memory_mib: val.memory_mib,
        }
    }
}
