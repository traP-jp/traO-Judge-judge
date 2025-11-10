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
}

pub struct InstancePool<AF, GF> {
    receiver: mpsc::UnboundedReceiver<InstancePoolMessage>,
    instance_tx: async_channel::Sender<InstanceMessage>,
    instance_rx: async_channel::Receiver<InstanceMessage>,
    reservation_count: usize,
    actual_instance_count: usize,
    aws_client_factory: AF,
    grpc_client_factory: GF,
}

impl<A, G, AFut, GFut, AF, GF> InstancePool<AF, GF>
where
    A: AwsClient + Send,
    G: GrpcClient + Send,
    AFut: Future<Output = A> + Send,
    GFut: Future<Output = G> + Send,
    AF: Fn() -> AFut + Send + Clone + 'static,
    GF: Fn(Ipv4Addr) -> GFut + Send + Clone + 'static,
{
    pub async fn new(
        receiver: mpsc::UnboundedReceiver<InstancePoolMessage>,
        aws_client_factory: AF,
        grpc_client_factory: GF,
    ) -> Self {
        let (instance_tx, instance_rx) = async_channel::unbounded();
        Self {
            receiver,
            instance_tx,
            instance_rx,
            reservation_count: 0,
            actual_instance_count: 0,
            aws_client_factory,
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
                let result = self
                    .handle_execution(reservation, outcome_id_for_res, dependencies)
                    .await;
                let _ = respond_to.send(result); // if this send fails, so does the recv.await after
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
            let aws_client_factory = self.aws_client_factory.clone();
            let grpc_client_factory = self.grpc_client_factory.clone();
            tokio::spawn(async move {
                Instance::new(instance_rx, aws_client_factory, grpc_client_factory)
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
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        tracing::debug!("[InstancePool::handle_execution] BEGIN");
        let (tx, rx) = oneshot::channel();
        let _ = self
            .instance_tx
            .send(InstanceMessage::Execution {
                outcome_id_for_res,
                dependencies,
                respond_to: tx,
            })
            .await; // if this send fails, so does the recv.await below
        let result = rx.await.map_err(|e| {
            tracing::error!("Instance task has been killed: {e}");
            job::ExecutionError::InternalError(format!("Instance task has been killed: {e}"))
        })?;

        drop(reservation);
        self.reservation_count -= 1;
        while self.actual_instance_count > self.desired_instance_count() {
            self.actual_instance_count -= 1;
            let (tx, rx) = oneshot::channel();
            let _ = self
                .instance_tx
                .send(InstanceMessage::Terminate { respond_to: tx })
                .await; // if this send fails, so does the recv.await below
            rx.await
                .map_err(|e| {
                    tracing::error!("Instance task has been killed: {e}");
                    job::ExecutionError::InternalError(format!(
                        "Instance task has been killed: {e}"
                    ))
                })?
                .map_err(|e| {
                    tracing::error!("Something went wrong on AWS client: {e}");
                    job::ExecutionError::InternalError(format!("AWSError: {e}"))
                })?;
        }
        tracing::debug!("[InstancePool::handle_execution] END");
        result
    }
    fn desired_instance_count(&self) -> usize {
        // TODO: consider this function
        self.reservation_count
    }
}
