use axum::async_trait;
use judge_core::model::job;

/// `JobService` manages job queuing and processing lazily.
#[async_trait]
pub trait JobService {
    /// Subscribe enqueued job by id if exists.
    async fn subscribe(
        &self,
        id: uuid::Uuid,
    ) -> Option<tokio::sync::oneshot::Receiver<JobResponse>>;

    /// Eagerly start a job by id.
    async fn start(
        &self,
        id: uuid::Uuid,
        job: JobRequest,
    ) -> tokio::sync::oneshot::Receiver<JobResponse>;

    /// Lazyly start a job by id.
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
