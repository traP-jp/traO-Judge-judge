use crate::*;
use anyhow::Result;
use axum::async_trait;
use judge_core::model::judge;
use tracing;

#[derive(Debug, Clone)]
pub struct WrappedJudgeApi<Inner: judge::JudgeApi> {
    inner_api: Inner,
}

impl<Inner: judge::JudgeApi> WrappedJudgeApi<Inner> {
    pub fn new(inner_api: Inner) -> Self {
        Self {
            inner_api: inner_api,
        }
    }
}

#[async_trait]
impl<Inner: judge::JudgeApi> generated::judge_service_server::JudgeService
    for WrappedJudgeApi<Inner>
{
    async fn judge(
        &self,
        request: tonic::Request<generated::JudgeRequest>,
    ) -> Result<tonic::Response<generated::JudgeResponse>, tonic::Status> {
        let request = request.into_inner();
        tracing::info!("Received request: {:?}", request);
        let request: judge::JudgeRequest = request
            .try_into()
            .map_err(|e| tonic::Status::invalid_argument(format!("Invalid request: {}", e)))?;
        let response = self
            .inner_api
            .judge(request)
            .await
            .map_err(|e| tonic::Status::internal(format!("Internal error: {}", e)))?;
        tracing::info!("Sending response: {:?}", response);
        let response = Ok(response);
        let response = response.into();
        Ok(tonic::Response::new(response))
    }
}
