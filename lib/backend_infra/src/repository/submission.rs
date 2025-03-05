use crate::model::submission::{JudgeResultRow, SubmissionRow};
use axum::async_trait;
use domain::{
    model::submission::{JudgeResult, Submission},
    repository::submission::SubmissionRepository,
};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct SubmissionRepositoryImpl {
    pool: MySqlPool,
}

impl SubmissionRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryImpl {
    async fn get_submission(&self, id: i64) -> anyhow::Result<Option<Submission>> {
        let submission =
            sqlx::query_as::<_, SubmissionRow>("SELECT * FROM submissions WHERE id = ?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(submission.map(|submission| submission.into()))
    }

    async fn get_submission_results(&self, id: i64) -> anyhow::Result<Vec<JudgeResult>> {
        let results = sqlx::query_as::<_, JudgeResultRow>(
            "SELECT * FROM submission_testcases WHERE submission_id = ?",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results.into_iter().map(|result| result.into()).collect())
    }
}
