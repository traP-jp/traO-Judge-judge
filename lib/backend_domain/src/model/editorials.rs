use async_session::chrono;

#[derive(Debug, Clone)]
pub struct Editorial {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
    pub statement: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
}
