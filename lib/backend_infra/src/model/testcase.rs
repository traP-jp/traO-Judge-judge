use domain::model::testcase::TestcaseSammary;
use sqlx::types::chrono;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TestcaseSammaryRow {
    pub id: i64,
    pub name: String,
    pub problem_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<TestcaseSammaryRow> for TestcaseSammary {
    fn from(val: TestcaseSammaryRow) -> Self {
        TestcaseSammary {
            id: val.id,
            name: val.name,
            problem_id: val.problem_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
