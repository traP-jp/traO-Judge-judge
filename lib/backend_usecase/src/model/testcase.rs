use domain::model::testcase::TestcaseSammary;
use sqlx::types::chrono;

pub struct TestcaseSammaryDto {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSammary> for TestcaseSammaryDto {
    fn from(testcase: TestcaseSammary) -> Self {
        Self {
            id: testcase.id,
            name: testcase.name,
            created_at: testcase.created_at,
            updated_at: testcase.updated_at,
        }
    }
}

pub struct TestcaseDto {
    pub id: i64,
    pub name: String,
    pub input: String,
    pub output: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


