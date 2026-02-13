use crate::model::problem::ProblemId;
use sqlx::types::chrono;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct TestcaseId(pub(crate) Uuid);

impl std::fmt::Display for TestcaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Uuid> for TestcaseId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TestcaseId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

pub struct TestcaseSummary {
    pub id: TestcaseId,
    pub name: String,
    pub problem_id: ProblemId,
    pub input_id: Uuid,
    pub output_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateTestcase {
    pub id: TestcaseId,
    pub name: String,
    pub problem_id: ProblemId,
    pub input_id: Uuid,
    pub output_id: Uuid,
}
