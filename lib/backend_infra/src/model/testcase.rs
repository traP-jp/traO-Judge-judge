use domain::model::testcase::TestcaseSummary;
use sqlx::types::chrono;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TestcaseRow {
    pub id: Uuid,
    pub name: String,
    pub problem_id: i64,
    pub input_id: Uuid,
    pub output_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseRow> for TestcaseSummary {
    fn from(val: TestcaseRow) -> Self {
        TestcaseSummary {
            id: val.id,
            name: val.name,
            problem_id: val.problem_id,
            input_id: val.input_id,
            output_id: val.output_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
