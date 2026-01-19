use domain::model::submission::Submission;
use sqlx::types::chrono;

#[derive(Debug, Clone)]
pub struct SubmissionDto {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub problem_id: String,
    pub problem_title: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub language_id: String,
    pub total_score: i64,
    pub max_time_ms: i32,
    pub max_memory_kib: i32,
    pub source_code: String,
    pub code_length: i32,
    pub overall_judge_status: String,
    pub judge_results: Vec<JudgeResultDto>,
}

#[derive(Debug, Clone)]
pub struct JudgeResultDto {
    pub testcase_id: String,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time_ms: i32,
    pub memory_kib: i32,
}

#[derive(Debug, Clone)]
pub struct SubmissionSummaryDto {
    pub id: String,
    pub problem_id: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub language_id: String,
    pub total_score: i64,
    pub max_time_ms: i32,
    pub max_memory_kib: i32,
    pub code_length: i32,
    pub judge_status: String,
}

impl From<Submission> for SubmissionSummaryDto {
    fn from(submission: Submission) -> Self {
        SubmissionSummaryDto {
            id: submission.id.to_string(),
            problem_id: submission.problem_id.to_string(),
            problem_title: submission.problem_title,
            user_id: submission.user_id.to_string(),
            user_name: submission.user_name,
            submitted_at: submission.submitted_at,
            language_id: submission.language_id,
            total_score: submission.total_score,
            max_time_ms: submission.max_time_ms,
            max_memory_kib: submission.max_memory_kib,
            code_length: submission.source.len() as i32,
            judge_status: submission.overall_judge_status,
        }
    }
}

pub struct SubmissionsDto {
    pub total: i64,
    pub submissions: Vec<SubmissionSummaryDto>,
}

pub enum SubmissionOrderByData {
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

pub struct SubmissionGetQueryData {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub judge_status: Option<String>,
    pub language: Option<String>,
    pub user_name: Option<String>,
    pub user_query: Option<String>,
    pub order_by: SubmissionOrderByData,
    pub problem_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateSubmissionData {
    pub language_id: String,
    pub source: String,
}
