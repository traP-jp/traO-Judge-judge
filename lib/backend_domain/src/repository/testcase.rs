use axum::async_trait;

use crate::model::testcase::TestcaseSummary;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait TestcaseRepository {
    async fn get_testcases(&self, problem_id: i64) -> anyhow::Result<Vec<TestcaseSummary>>;
    async fn get_testcase(&self, id: i64) -> anyhow::Result<Option<TestcaseSummary>>;
    async fn update_testcase_name(&self, id: i64, name: String) -> anyhow::Result<()>;
    async fn create_testcases(&self, problem_id: i64, names: Vec<String>) -> anyhow::Result<()>;
    async fn delete_testcase(&self, id: i64) -> anyhow::Result<()>;
}
