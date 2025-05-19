use judge_core::model::job;
use judge_exec_grpc::generated::{
    execute_service_client::ExecuteServiceClient, Dependency, ExecuteRequest,
};
use std::{net::Ipv4Addr, os::unix::process::ExitStatusExt};
use uuid::Uuid;

use crate::jobapi::OutcomeToken;

#[axum::async_trait]
pub trait GrpcClient {
    async fn execute(
        &mut self,
        outcome_id_for_res: Uuid,
        dependency: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError>;
}

pub struct GrpcClientType {
    exec_client: ExecuteServiceClient<tonic::transport::Channel>,
}

impl GrpcClientType {
    pub async fn try_new(instance_ip: Ipv4Addr) -> Result<Self, tonic::transport::Error> {
        ExecuteServiceClient::connect(format!("http://{}:{}", instance_ip, 50051))
            .await
            .map(|exec_client| Self { exec_client })
    }
}

#[axum::async_trait]
impl GrpcClient for GrpcClientType {
    async fn execute(
        &mut self,
        outcome_id_for_res: Uuid,
        dependency: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        let mut request = vec![];
        for job::Dependency { envvar, outcome } in dependency {
            request.push(Dependency {
                envvar,
                outcome_uuid: outcome.outcome_id.to_string(),
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
            .unwrap()
            .into_inner();

        tracing::info!("Execute response received: {:?}", resp);

        let outcome = resp.outcome;
        let output = resp.output.unwrap();
        Ok((
            OutcomeToken::from_binary(outcome_id_for_res, &outcome).await,
            std::process::Output {
                status: ExitStatusExt::from_raw(output.exit_code),
                stdout: output.stdout.as_bytes().to_vec(),
                stderr: output.stderr.as_bytes().to_vec(),
            },
        ))
    }
}
