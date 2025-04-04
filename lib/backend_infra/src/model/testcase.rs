use domain::model::testcase::TestcaseSummary;
use sqlx::types::chrono;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TestcaseSummaryRow {
    pub id: i64,
    pub name: String,
    pub problem_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSummaryRow> for TestcaseSummary {
    fn from(val: TestcaseSummaryRow) -> Self {
        TestcaseSummary {
            id: val.id,
            name: val.name,
            problem_id: val.problem_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
