use again::RetryPolicy;
use judge_core::model::job;
use judge_exec_grpc::generated::{
    execute_service_client::ExecuteServiceClient, Dependency, ExecuteRequest,
};
use std::{error::Error, net::Ipv4Addr, os::unix::process::ExitStatusExt, time::Duration};
use uuid::Uuid;

use crate::jobapi::OutcomeToken;

#[axum::async_trait]
pub trait GrpcClient {
    async fn execute(
        &mut self,
        dependency: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError>;
}

pub struct GrpcClientType {
    exec_client: ExecuteServiceClient<tonic::transport::Channel>,
}

impl GrpcClientType {
    pub async fn new(instance_ip: Ipv4Addr) -> Self {
        // 1 秒間隔で 60 回ポーリング
        let policy = RetryPolicy::fixed(Duration::from_secs(1)).with_max_retries(60);
        let exec_client = policy
            .retry_if(
                || async move {
                    ExecuteServiceClient::connect(format!("http://{}:{}", instance_ip, 50051)).await
                },
                |e: &tonic::transport::Error| {
                    // ConnectionRefused のときのみリトライ
                    e.source()
                        .and_then(|s| s.source())
                        .and_then(|s| s.source())
                        .and_then(|s| s.downcast_ref::<std::io::Error>())
                        .map(|e| e.kind())
                        .is_some_and(|k| k == std::io::ErrorKind::ConnectionRefused)
                },
            )
            .await
            .unwrap(); // unexpected
        Self { exec_client }
    }
}

#[axum::async_trait]
impl GrpcClient for GrpcClientType {
    async fn execute(
        &mut self,
        dependency: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        let mut request = vec![];
        for job::Dependency { envvar, outcome } in dependency {
            request.push(Dependency {
                envvar,
                outcome: outcome.to_binary().await.to_vec(),
            })
        }
        let resp = self
            .exec_client
            .execute(ExecuteRequest {
                dependency: request,
                exec_time_ms: 998244353,
            })
            .await
            .unwrap();
        let outcome = resp.get_ref().clone().outcome;
        let outcome_id = Uuid::now_v7();
        let output = resp.get_ref().clone().output.unwrap();
        Ok((
            OutcomeToken::from_binary(outcome_id, &outcome).await,
            std::process::Output {
                status: ExitStatusExt::from_raw(output.exit_code),
                stdout: output.stdout.as_bytes().to_vec(),
                stderr: output.stderr.as_bytes().to_vec(),
            },
        ))
    }
}
