use axum::async_trait;

use crate::model::problem::ProblemId;
use crate::model::testcase::{CreateTestcase, TestcaseId, TestcaseSummary};

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait TestcaseRepository {
    async fn get_testcases(&self, problem_id: ProblemId) -> anyhow::Result<Vec<TestcaseSummary>>;
    async fn get_testcase(&self, id: TestcaseId) -> anyhow::Result<Option<TestcaseSummary>>;
    async fn create_testcases(&self, testcases: Vec<CreateTestcase>) -> anyhow::Result<()>;
    async fn delete_testcases(&self, problem_id: ProblemId) -> anyhow::Result<()>;
}
