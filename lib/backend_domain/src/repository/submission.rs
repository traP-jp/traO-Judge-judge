use axum::async_trait;

use crate::model::submission::{
    CreateJudgeResult, CreateSubmission, JudgeResult, Submission, SubmissionGetQuery, SubmissionId,
    UpdateSubmission,
};

#[cfg_attr(feature = "mockall", mockall::automock)]
#[async_trait]
pub trait SubmissionRepository {
    async fn get_submission(&self, id: SubmissionId) -> anyhow::Result<Option<Submission>>;
    async fn get_submission_results(&self, id: SubmissionId) -> anyhow::Result<Vec<JudgeResult>>;
    async fn get_submissions_by_query(
        &self,
        query: SubmissionGetQuery,
    ) -> anyhow::Result<Vec<Submission>>;
    async fn get_submissions_count_by_query(
        &self,
        query: SubmissionGetQuery,
    ) -> anyhow::Result<i64>;
    async fn create_submission(&self, submission: CreateSubmission)
    -> anyhow::Result<SubmissionId>;
    async fn update_submission(
        &self,
        submission_id: SubmissionId,
        submission: UpdateSubmission,
    ) -> anyhow::Result<()>;
    async fn create_judge_results(&self, results: Vec<CreateJudgeResult>) -> anyhow::Result<()>;
    async fn delete_judge_results_by_submission_id(
        &self,
        submission_id: SubmissionId,
    ) -> anyhow::Result<()>;
}
