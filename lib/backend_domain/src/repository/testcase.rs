use axum::async_trait;

use crate::model::testcase::TestcaseSammary;

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait TestcaseRepository {
    async fn get_testcases(&self, problem_id: i64) -> anyhow::Result<Vec<TestcaseSammary>>;
    async fn get_testcase(&self, id: i64) -> anyhow::Result<Option<TestcaseSammary>>;
    async fn update_testcase_name(&self, id: i64, name: String) -> anyhow::Result<()>;
    async fn create_testcases(&self, problem_id: i64, names: Vec<String>) -> anyhow::Result<()>;
    async fn delete_testcase(&self, id: i64) -> anyhow::Result<()>;
}
