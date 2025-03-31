use sqlx::types::chrono;

use domain::model::submission::{JudgeResult, Submission};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SubmissionRow {
    pub id: i64,
    pub problem_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub language_id: i32,
    pub source: String,
    pub judge_status: String,
    pub total_score: i64,
    pub max_time: i32,
    pub max_memory: i32,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

impl From<SubmissionRow> for Submission {
    fn from(val: SubmissionRow) -> Self {
        Submission {
            id: val.id,
            problem_id: val.problem_id,
            user_id: val.user_id,
            user_name: val.user_name,
            language_id: val.language_id,
            source: val.source,
            overall_judge_status: val.judge_status,
            total_score: val.total_score,
            max_time: val.max_time,
            max_memory: val.max_memory,
            submitted_at: val.submitted_at,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct JudgeResultRow {
    pub submission_id: i64,
    pub testcase_id: i64,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time: i32,
    pub memory: i32,
}

impl From<JudgeResultRow> for JudgeResult {
    fn from(val: JudgeResultRow) -> Self {
        JudgeResult {
            testcase_id: val.testcase_id,
            testcase_name: val.testcase_name,
            judge_status: val.judge_status,
            score: val.score,
            time: val.time,
            memory: val.memory,
        }
    }
}
