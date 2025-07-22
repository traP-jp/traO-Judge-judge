use super::uuid::UuidRow;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserAuthenticationRow {
    pub user_id: UuidRow,
    pub password: Option<String>,
    pub github_oauth: Option<String>,
    pub google_oauth: Option<String>,
    pub traq_oauth: Option<String>,
}
