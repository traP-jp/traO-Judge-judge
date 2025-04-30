use domain::model::testcase::TestcaseSummary;
use sqlx::types::chrono;
use uuid::Uuid;

pub struct TestcaseSummaryDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSummary> for TestcaseSummaryDto {
    fn from(testcase: TestcaseSummary) -> Self {
        Self {
            id: testcase.id,
            name: testcase.name,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}

pub struct TestcaseDto {
    pub id: Uuid,
    pub name: String,
    pub input: String,
    pub output: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateTestcaseData {
    pub name: String,
    pub input: String,
    pub output: String,
}
