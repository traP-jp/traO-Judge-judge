use axum::async_trait;
use domain::{
    model::testcase::{CreateTestcase, TestcaseSummary},
    repository::testcase::TestcaseRepository,
};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::model::{testcase::TestcaseRow, uuid::UuidRow};

#[derive(Clone)]
pub struct TestcaseRepositoryImpl {
    pool: MySqlPool,
}

impl TestcaseRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TestcaseRepository for TestcaseRepositoryImpl {
    async fn get_testcases(&self, problem_id: i64) -> anyhow::Result<Vec<TestcaseSummary>> {
        let testcases =
            sqlx::query_as::<_, TestcaseRow>("SELECT * FROM `testcases` WHERE `problem_id` = ?")
                .bind(problem_id)
                .fetch_all(&self.pool)
                .await?;

        Ok(testcases.into_iter().map(|row| row.into()).collect())
    }

    async fn get_testcase(&self, id: Uuid) -> anyhow::Result<Option<TestcaseSummary>> {
        let testcase = sqlx::query_as::<_, TestcaseRow>("SELECT * FROM `testcases` WHERE `id` = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(testcase.map(|row| row.into()))
    }

    async fn create_testcases(&self, testcases: Vec<CreateTestcase>) -> anyhow::Result<()> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO `testcases` (`id`, `problem_id`, `name`, `input_id`, `output_id`) VALUES ",
        );

        let mut separated = query_builder.separated(", ");
        for testcase in testcases {
            separated.push("(");
            separated.push_bind_unseparated(UuidRow(testcase.id));
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(testcase.problem_id);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(testcase.name);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(UuidRow(testcase.input_id));
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(UuidRow(testcase.output_id));
            separated.push_unseparated(")");
        }

        let query = query_builder.build();
        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_testcases(&self, problem_id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM `testcases` WHERE `problem_id` = ?")
            .bind(problem_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
