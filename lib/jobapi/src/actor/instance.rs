use std::future::Future;
use std::net::Ipv4Addr;

use judge_core::model::job;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::actor::Running;
use crate::jobapi::OutcomeToken;
use crate::model::aws::AwsClient;
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
    instance_id: Uuid,
    receiver: async_channel::Receiver<InstanceMessage>, // multi-consumer
    aws_client: A,
    grpc_client: G,
}

impl<A: AwsClient, G: GrpcClient> Instance<A, G> {
    pub async fn new<AFut, GFut, AF, GF>(
        receiver: async_channel::Receiver<InstanceMessage>,
        aws_client_factory: AF,
        grpc_client_factory: GF,
    ) -> Self
    where
        AFut: Future<Output = A>,
        GFut: Future<Output = G>,
        AF: Fn() -> AFut,
        GF: Fn(Ipv4Addr) -> GFut,
    {
        let instance_id = Uuid::now_v7();
        // warm-up AWS & gRPC client
        let mut aws_client = aws_client_factory().await;
        let instance_ip = aws_client.create_instance(instance_id).await.unwrap();
        let grpc_client = grpc_client_factory(instance_ip).await;
        Self {
            instance_id,
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
                let result = self
                    .grpc_client
                    .execute(outcome_id_for_res, dependencies)
                    .await;
                let _ = respond_to.send(result); // if this send fails, so does the recv.await after
                Running::Continue
            }
            InstanceMessage::Terminate { respond_to } => {
                let result = self.aws_client.terminate_instance(self.instance_id).await;
                let _ = respond_to.send(result); // if this send fails, so does the recv.await after
                Running::Stop
            }
        }
    }
}
