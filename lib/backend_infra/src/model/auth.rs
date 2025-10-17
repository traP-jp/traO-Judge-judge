use domain::model::auth::UserAuthentication;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserAuthenticationRow {
    pub email: Option<String>,
    pub github_oauth: Option<String>,
    pub google_oauth: Option<String>,
    pub traq_oauth: Option<String>,
}

impl From<UserAuthenticationRow> for UserAuthentication {
    fn from(row: UserAuthenticationRow) -> Self {
        UserAuthentication {
            email: row.email,
            github_oauth: row.github_oauth,
            google_oauth: row.google_oauth,
            traq_oauth: row.traq_oauth,
        }
    }
}
