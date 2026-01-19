use std::future::Future;
use std::net::Ipv4Addr;

use judge_core::model::job;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::actor::Running;
use crate::job_service::OutcomeToken;
use crate::model::aws::{AwsClient, AwsInstanceInfo};
use crate::model::grpc::GrpcClient;

pub enum InstanceMessage {
    Execution {
        outcome_id_for_res: Uuid,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
        respond_to:
            oneshot::Sender<Result<(OutcomeToken, std::process::Output), job::ExecutionError>>,
    },
    Terminate {
        respond_to: oneshot::Sender<Result<(), anyhow::Error>>,
    },
}

pub struct Instance<A, G> {
    aws_id: String,
    receiver: async_channel::Receiver<InstanceMessage>, // multi-consumer
    aws_client: A,
    grpc_client: G,
}

impl<A: AwsClient, G: GrpcClient> Instance<A, G> {
    pub async fn new<GFut, GF>(
        receiver: async_channel::Receiver<InstanceMessage>,
        aws_client: A,
        grpc_client_factory: GF,
    ) -> Self
    where
        GFut: Future<Output = G>,
        GF: Fn(Ipv4Addr) -> GFut,
    {
        tracing::debug!("[Instance::new] BEGIN");

        // warm-up AWS & gRPC client
        tracing::debug!("[Instance::new] create instance BEGIN");
        let AwsInstanceInfo { aws_id, ip_addr } = aws_client.create_instance().await.unwrap();
        tracing::debug!("[Instance::new] create instance END");

        tracing::debug!("[Instance::new] gen grpc BEGIN aws_id={}", aws_id);
        let grpc_client = grpc_client_factory(ip_addr).await;
        tracing::debug!("[Instance::new] gen grpc END aws_id={}", aws_id);

        tracing::debug!("[Instance::new] END aws_id={}", aws_id);
        Self {
            aws_id,
            receiver,
            aws_client,
            grpc_client,
        }
    }
    pub async fn run(&mut self) {
        while let Ok(msg) = self.receiver.recv().await {
            let running = self.handle(msg).await;
            match running {
                Running::Continue => continue,
                Running::Stop => break,
            }
        }
    }
    async fn handle(&mut self, msg: InstanceMessage) -> Running {
        match msg {
            InstanceMessage::Execution {
                outcome_id_for_res,
                dependencies,
                respond_to,
            } => {
                tracing::debug!("[Instance::handle_execution] BEGIN aws_id={}", self.aws_id);
                let result = self
                    .grpc_client
                    .execute(outcome_id_for_res, dependencies)
                    .await;
                tracing::debug!("[Instance::handle_execution] END aws_id={}", self.aws_id);
                let _ = respond_to.send(result); // if this send fails, so does the recv.await after
                Running::Continue
            }
            InstanceMessage::Terminate { respond_to } => {
                tracing::debug!("[Instance::handle_terminate] BEGIN aws_id={}", self.aws_id);
                let result = self
                    .aws_client
                    .terminate_instance(self.aws_id.clone())
                    .await;
                tracing::debug!("[Instance::handle_terminate] END aws_id={}", self.aws_id);
                let _ = respond_to.send(result); // if this send fails, so does the recv.await after
                Running::Stop
            }
        }
    }
}
