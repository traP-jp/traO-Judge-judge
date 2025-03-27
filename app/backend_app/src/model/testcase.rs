use async_session::chrono;
use serde::{Deserialize, Serialize};
use usecase::model::testcase::TestcaseSummaryDto;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseSummary {
    pub id: i64,
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
