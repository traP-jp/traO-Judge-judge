use axum::async_trait;

use crate::model::submisson::{JudgeResult, Submission};

#[async_trait]
pub trait SubmissionRepository {
    async fn get_submission(&self, id: i64) -> anyhow::Result<Option<Submission>>;
    async fn get_submission_results(&self, id: i64) -> anyhow::Result<Vec<JudgeResult>>;
}
