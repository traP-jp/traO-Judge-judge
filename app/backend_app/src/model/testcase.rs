use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use usecase::model::testcase::TestcaseSummaryDto;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseSummary {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSummaryDto> for TestcaseSummary {
    fn from(testcase: TestcaseSummaryDto) -> Self {
        TestcaseSummary {
            id: testcase.id,
            name: testcase.name,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}
