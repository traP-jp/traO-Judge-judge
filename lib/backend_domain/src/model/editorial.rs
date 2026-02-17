use crate::model::{problem::ProblemId, user::UserDisplayId};
use async_session::chrono;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EditorialId(Uuid);

impl std::fmt::Display for EditorialId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Uuid> for EditorialId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for EditorialId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct Editorial {
    pub id: EditorialId,
    pub problem_id: ProblemId,
    pub author_id: UserDisplayId,
    pub title: String,
    pub statement: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct CreateEditorial {
    pub problem_id: ProblemId,
    pub author_id: UserDisplayId,
    pub title: String,
    pub statement: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateEditorial {
    pub id: EditorialId,
    pub title: String,
    pub statement: String,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct EditorialSummary {
    pub id: EditorialId,
    pub problem_id: ProblemId,
    pub author_id: UserDisplayId,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct EditorialGetQuery {
    pub user_id: Option<UserDisplayId>,
    pub problem_id: ProblemId,
    pub limit: i64,
    pub offset: i64,
}
