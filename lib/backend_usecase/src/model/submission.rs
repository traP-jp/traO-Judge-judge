use sqlx::types::chrono;

#[derive(Debug, Clone)]
pub struct SubmissionDto {
    pub id: String,
    pub user_id: i64,
    pub user_name: String,
    pub problem_id: i64,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub language_id: i32,
    pub total_score: i64,
    pub max_time: i32,
    pub max_memory: i32,
    pub code_length: i32,
    pub overall_judge_status: String,
    pub judge_results: Vec<JudgeResultDto>,
}

#[derive(Debug, Clone)]
pub struct JudgeResultDto {
    pub testcase_id: i64,
    pub testcase_name: String,
    pub judge_status: String,
    pub score: i64,
    pub time: i32,
    pub memory: i32,
}
