use async_trait::async_trait;
use judge_core::model::job;
use judge_exec_grpc::generated::{
    execute_service_client::ExecuteServiceClient, Dependency, ExecuteRequest,
};
use std::{net::Ipv4Addr, os::unix::process::ExitStatusExt};
use uuid::Uuid;

use crate::jobapi::OutcomeToken;

#[async_trait]
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
        let exec_client =
            ExecuteServiceClient::connect(format!("http://{}:{}", instance_ip, 50051))
                .await
                .unwrap();
        Self { exec_client }
    }
}

#[async_trait]
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
