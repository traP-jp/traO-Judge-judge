use async_session::chrono;
use serde::{Deserialize, Serialize};
use usecase::model::testcase::TestcaseSammaryDto;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestcaseSammary {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSammaryDto> for TestcaseSammary {
    fn from(testcase: TestcaseSammaryDto) -> Self {
        TestcaseSammary {
            id: testcase.id,
            name: testcase.name,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}
