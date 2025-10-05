use domain::model::testcase::TestcaseSummary;
use sqlx::types::chrono;

use crate::model::uuid::UuidRow;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TestcaseRow {
    pub id: UuidRow,
    pub name: String,
    pub problem_id: i64,
    pub input_id: UuidRow,
    pub output_id: UuidRow,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseRow> for TestcaseSummary {
    fn from(val: TestcaseRow) -> Self {
        TestcaseSummary {
            id: val.id.0,
            name: val.name,
            problem_id: val.problem_id,
            input_id: val.input_id.0,
            output_id: val.output_id.0,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
