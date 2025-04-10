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

#[derive(Debug, Clone)]
pub struct CreateEditorial {
    pub problem_id: i64,
    pub author_id: i64,
    pub statement: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateEditorial {
    pub id: i64,
    pub statement: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct EditorialSummary {
    pub id: i64,
    pub problem_id: i64,
    pub author_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct EditorialGetQuery {
    pub user_id: Option<i64>,
    pub problem_id: i64,
    pub limit: i64,
    pub offset: i64,
}
