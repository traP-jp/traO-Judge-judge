use axum::async_trait;
use judge_core::model::job;

/// `JobService` manages job queuing and processing lazily.
#[async_trait]
pub trait JobService {
    async fn subscribe(
        &self,
        id: uuid::Uuid,
    ) -> Option<tokio::sync::oneshot::Receiver<JobResponse>>;

    async fn start(
        &self,
        id: uuid::Uuid,
        job: JobRequest,
    ) -> tokio::sync::oneshot::Receiver<JobResponse>;

    async fn subscribe_or_start(
        &self,
        id: uuid::Uuid,
        job: &JobRequest,
    ) -> tokio::sync::oneshot::Receiver<JobResponse> {
        self.subscribe(id)
            .await
            .unwrap_or(self.start(id, job.clone()).await)
    }
}
