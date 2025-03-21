use sqlx::types::chrono;

pub struct TestcaseSammary {
    pub id: i64,
    pub name: String,
    pub problem_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
