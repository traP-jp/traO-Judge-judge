use axum::async_trait;
use domain::{model::testcase::TestcaseSammary, repository::testcase::TestcaseRepository};
use sqlx::MySqlPool;

use crate::model::testcase::TestcaseSammaryRow;

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
    async fn get_testcases(&self, problem_id: i64) -> anyhow::Result<Vec<TestcaseSammary>> {
        let testcases = sqlx::query_as::<_, TestcaseSammaryRow>(
            "SELECT * FROM `testcases` WHERE `problem_id` = ?",
        )
        .bind(problem_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(testcases.into_iter().map(|row| row.into()).collect())
    }

    async fn get_testcase(&self, id: i64) -> anyhow::Result<Option<TestcaseSammary>> {
        let testcase =
            sqlx::query_as::<_, TestcaseSammaryRow>("SELECT * FROM `testcases` WHERE `id` = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(testcase.map(|row| row.into()))
    }

    async fn update_testcase_name(&self, id: i64, name: String) -> anyhow::Result<()> {
        sqlx::query("UPDATE `testcases` SET `name` = ? WHERE `id` = ?")
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn create_testcases(&self, problem_id: i64, names: Vec<String>) -> anyhow::Result<()> {
        let mut query_builder =
            sqlx::QueryBuilder::new("INSERT INTO `testcases` (`problem_id`, `name`) VALUES ");
        for name in names {
            query_builder.push(" (?, ?),");
            query_builder.push_bind(problem_id);
            query_builder.push_bind(name);
        }
        query_builder.push(";");

        let query = query_builder.build();
        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_testcase(&self, id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM `testcases` WHERE `id` = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
