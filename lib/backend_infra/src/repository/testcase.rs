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
        let testcases = sqlx::query_as!(
            TestcaseRow,
            r#"
                SELECT 
                    id AS "id: _",
                    name,
                    problem_id,
                    input_id AS "input_id: _",
                    output_id AS "output_id: _",
                    created_at AS "created_at: _",
                    updated_at AS "updated_at: _"
                FROM 
                    `testcases` 
                WHERE 
                    `problem_id` = ?
                "#,
            problem_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(testcases.into_iter().map(|row| row.into()).collect())
    }

    async fn get_testcase(&self, id: Uuid) -> anyhow::Result<Option<TestcaseSummary>> {
        let testcase = sqlx::query_as!(
            TestcaseRow,
            r#"
            SELECT
                id AS "id: _",
                name,
                problem_id,
                input_id AS "input_id: _",
                output_id AS "output_id: _",
                created_at AS "created_at: _",
                updated_at AS "updated_at: _"
            FROM 
                `testcases` 
            WHERE 
                `id` = ?
            "#,
            UuidRow(id)
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(testcase.map(|row| row.into()))
    }

    async fn create_testcases(&self, testcases: Vec<CreateTestcase>) -> anyhow::Result<()> {
        if testcases.is_empty() {
            return Ok(());
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
            INSERT INTO 
                `testcases` (
                    `id`, 
                    `problem_id`, 
                    `name`, 
                    `input_id`, 
                    `output_id`
                ) 
            VALUES 
            "#,
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
        sqlx::query!(
            r#"
            DELETE FROM 
                `testcases` 
            WHERE 
                `problem_id` = ?
            "#,
            problem_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
