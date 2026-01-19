use axum::async_trait;
use uuid::Uuid;

use crate::model::testcase::{CreateTestcase, TestcaseSummary};

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait TestcaseRepository {
    async fn get_testcases(&self, problem_id: i64) -> anyhow::Result<Vec<TestcaseSummary>>;
    async fn get_testcase(&self, id: Uuid) -> anyhow::Result<Option<TestcaseSummary>>;
    async fn create_testcases(&self, testcases: Vec<CreateTestcase>) -> anyhow::Result<()>;
    async fn delete_testcases(&self, problem_id: i64) -> anyhow::Result<()>;
}
