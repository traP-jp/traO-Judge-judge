use sqlx::types::chrono;
use uuid::Uuid;

pub struct TestcaseSummary {
    pub id: Uuid,
    pub name: String,
    pub problem_id: i64,
    pub input_id: Uuid,
    pub output_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateTestcase {
    pub id: Uuid,
    pub name: String,
    pub problem_id: i64,
    pub input_id: Uuid,
    pub output_id: Uuid,
}
