use std::{future::Future, net::Ipv4Addr};

use judge_core::model::*;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::job_service::{OutcomeToken, ReservationToken};
use crate::{
    actor::{
        Running,
        instance::{Instance, InstanceMessage},
    },
    model::{aws::AwsClient, grpc::GrpcClient},
};

pub enum InstancePoolMessage {
    Reservation {
        count: usize,
        respond_to: oneshot::Sender<Result<Vec<ReservationToken>, job::ReservationError>>,
    },
    Execution {
        reservation: ReservationToken,
        outcome_id_for_res: Uuid,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
        respond_to:
            oneshot::Sender<Result<(OutcomeToken, std::process::Output), job::ExecutionError>>,
    },
    Completion,
}

pub struct InstancePool<A, GF> {
    receiver: mpsc::UnboundedReceiver<InstancePoolMessage>,
    pool_tx: mpsc::UnboundedSender<InstancePoolMessage>,
    instance_tx: async_channel::Sender<InstanceMessage>,
    instance_rx: async_channel::Receiver<InstanceMessage>,
    reservation_count: usize,
    actual_instance_count: usize,
    aws_client: A,
    grpc_client_factory: GF,
}

impl<A, G, GFut, GF> InstancePool<A, GF>
where
    A: AwsClient + Send + Clone + 'static,
    G: GrpcClient + Send,
    GFut: Future<Output = G> + Send,
    GF: Fn(Ipv4Addr) -> GFut + Send + Clone + 'static,
{
    pub async fn new(
        receiver: mpsc::UnboundedReceiver<InstancePoolMessage>,
        pool_tx: mpsc::UnboundedSender<InstancePoolMessage>,
        aws_client: A,
        grpc_client_factory: GF,
    ) -> Self {
        let (instance_tx, instance_rx) = async_channel::unbounded();
        Self {
            receiver,
            pool_tx,
            instance_tx,
            instance_rx,
            reservation_count: 0,
            actual_instance_count: 0,
            aws_client,
            grpc_client_factory,
        }
    }
    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            let running = self.handle(msg).await;
            match running {
                Running::Continue => continue,
                Running::Stop => break,
            }
        }
    }
    async fn handle(&mut self, msg: InstancePoolMessage) -> Running {
        match msg {
            InstancePoolMessage::Reservation { count, respond_to } => {
                let result = self.handle_reservation(count).await;
                let _ = respond_to.send(result); // if this send fails, so does the recv.await after
                Running::Continue
            }
            InstancePoolMessage::Execution {
                reservation,
                outcome_id_for_res,
                dependencies,
                respond_to,
            } => {
                self.handle_execution(reservation, outcome_id_for_res, dependencies, respond_to)
                    .await;
                Running::Continue
            }
            InstancePoolMessage::Completion => {
                self.handle_completion().await;
                Running::Continue
            }
        }
    }
    async fn handle_reservation(
        &mut self,
        count: usize,
    ) -> Result<Vec<ReservationToken>, job::ReservationError> {
        tracing::debug!("[InstancePool::handle_reservation] BEGIN");
        let result = (0..count)
            .map(|_| {
                self.reservation_count += 1;
                ReservationToken {}
            })
            .collect();
        while self.actual_instance_count < self.desired_instance_count() {
            self.actual_instance_count += 1;
            let instance_rx = self.instance_rx.clone();
            let aws_client = self.aws_client.clone();
            let grpc_client_factory = self.grpc_client_factory.clone();
            tokio::spawn(async move {
                Instance::new(instance_rx, aws_client, grpc_client_factory)
                    .await
                    .run()
                    .await;
            });
        }
        tracing::debug!("[InstancePool::handle_reservation] END");
        Ok(result)
    }
    async fn handle_execution(
        &mut self,
        reservation: ReservationToken,
        outcome_id_for_res: Uuid,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
        respond_to: oneshot::Sender<
            Result<(OutcomeToken, std::process::Output), job::ExecutionError>,
        >,
    ) {
        tracing::debug!("[InstancePool::handle_execution] BEGIN");
        let instance_tx = self.instance_tx.clone();
        let pool_tx = self.pool_tx.clone();
        tokio::spawn(async move {
            let (tx, rx) = oneshot::channel();
            let _ = instance_tx
                .send(InstanceMessage::Execution {
                    outcome_id_for_res,
                    dependencies,
                    respond_to: tx,
                })
                .await; // if this send fails, so does the recv.await after

            let result = rx
                .await
                .map_err(|e| {
                    tracing::error!("Instance task has been killed: {e}");
                    job::ExecutionError::InternalError(format!(
                        "Instance task has been killed: {e}"
                    ))
                })
                .and_then(|res| res);
            let _ = respond_to.send(result); // if this send fails, so does the recv.await after

            drop(reservation);
            let _ = pool_tx.send(InstancePoolMessage::Completion); // if this send fails, so does the recv.await after
        });
    }

    async fn handle_completion(&mut self) {
        tracing::debug!("[InstancePool::handle_completion] BEGIN");
        self.reservation_count = self.reservation_count.saturating_sub(1);
        while self.actual_instance_count > self.desired_instance_count() {
            self.actual_instance_count -= 1;
            let (tx, rx) = oneshot::channel();
            let _ = self
                .instance_tx
                .send(InstanceMessage::Terminate { respond_to: tx })
                .await; // if this send fails, so does the recv.await after
            tokio::spawn(async move {
                match rx.await {
                    Ok(Err(e)) => tracing::error!("AWSError during termination: {e}"),
                    Err(e) => tracing::error!("Instance task killed during termination: {e}"),
                    _ => (),
                }
            });
        }
        tracing::debug!("[InstancePool::handle_completion] END");
    }

    fn desired_instance_count(&self) -> usize {
        // TODO: consider this function
        self.reservation_count.min(1)
    }
}
