use async_session::chrono;
use uuid::Uuid;

pub struct Submission {
    pub id: Uuid,
    pub user_id: i64,
    pub user_name: String,
    pub problem_id: i64,
    pub problem_title: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub language_id: i32,
    pub total_score: i64,
    pub max_time_ms: i32,
    pub max_memory_mib: i32,
    pub source: String,
    pub overall_judge_status: String,
}

pub struct JudgeResult {
    pub testcase_id: Uuid,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time_ms: i32,
    pub memory_mib: i32,
}

pub struct CreateSubmission {
    pub problem_id: i64,
    pub user_id: i64,
    pub language_id: i32,
    pub source: String,
    pub judge_status: String,
    pub total_score: i64,
    pub max_time_ms: i32,
    pub max_memory_mib: i32,
}

pub struct UpdateSubmission {
    pub judge_status: String,
    pub total_score: i64,
    pub max_time_ms: i32,
    pub max_memory_mib: i32,
}

pub struct CreateJudgeResult {
    pub submission_id: Uuid,
    pub testcase_id: Uuid,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time_ms: i32,
    pub memory_mib: i32,
}

#[derive(Clone)]
pub enum SubmissionOrderBy {
    SubmittedAtAsc,
    SubmittedAtDesc,
    TimeConsumptionAsc,
    TimeConsumptionDesc,
    ScoreAsc,
    ScoreDesc,
    MemoryConsumptionAsc,
    MemoryConsumptionDesc,
    CodeLengthAsc,
    CodeLengthDesc,
}

#[derive(Clone)]
pub struct SubmissionGetQuery {
    pub user_id: Option<i64>,
    pub limit: i64,
    pub offset: i64,
    pub judge_status: Option<String>,
    pub language_id: Option<i64>,
    pub user_name: Option<String>,
    pub user_query: Option<i64>,
    pub order_by: SubmissionOrderBy,
    pub problem_id: Option<i64>,
}
