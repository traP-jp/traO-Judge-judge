use crate::*;
use anyhow::Result;
use judge_core::model::judge;

#[derive(Debug, Clone)]
pub struct RemoteJudgeApiClient {
    grpc_client: generated::judge_service_client::JudgeServiceClient<tonic::transport::Channel>,
}

impl RemoteJudgeApiClient {
    pub async fn new(uri: &str) -> anyhow::Result<Self> {
        let channel = tonic::transport::Channel::from_shared(uri.to_string())?
            .connect()
            .await?;
        let grpc_client = generated::judge_service_client::JudgeServiceClient::new(channel);
        Ok(Self { grpc_client })
    }
}

#[axum::async_trait]
impl judge::JudgeApi for RemoteJudgeApiClient {
    async fn judge(&self, request: judge::JudgeRequest) -> judge::JudgeResponse {
        let grpc_request: generated::JudgeRequest = request.into();
        let mut grpc_client = self.grpc_client.clone();
        let grpc_response = grpc_client
            .judge(tonic::Request::new(grpc_request))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to execute remote judge: {}", e))?
            .into_inner();
        let response: judge::JudgeResponse = grpc_response.into();
        response
    }
}
