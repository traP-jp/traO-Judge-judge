use judge_core::model::*;
use tokio::sync::{mpsc, oneshot};

use crate::actor::{
    instance::{Instance, InstanceMessage},
    Running,
};
use crate::jobapi::{OutcomeToken, ReservationToken};

pub enum InstancePoolMessage {
    Reservation {
        count: usize,
        respond_to: oneshot::Sender<Result<Vec<ReservationToken>, job::ReservationError>>,
    },
    Execution {
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
        respond_to:
            oneshot::Sender<Result<(OutcomeToken, std::process::Output), job::ExecutionError>>,
    },
}

pub struct InstancePool {
    receiver: mpsc::UnboundedReceiver<InstancePoolMessage>,
    instance_tx: async_channel::Sender<InstanceMessage>,
    instance_rx: async_channel::Receiver<InstanceMessage>,
    reservation_count: usize,
    actual_instance_count: usize,
}

impl InstancePool {
    pub async fn new(receiver: mpsc::UnboundedReceiver<InstancePoolMessage>) -> Self {
        let (instance_tx, instance_rx) = async_channel::unbounded();
        Self {
            receiver,
            instance_tx,
            instance_rx,
            reservation_count: 0,
            actual_instance_count: 0,
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
                respond_to.send(result).unwrap();
                Running::Continue
            }
            InstancePoolMessage::Execution {
                reservation,
                dependencies,
                respond_to,
            } => {
                let result = self.handle_execution(reservation, dependencies).await;
                respond_to.send(result).unwrap();
                Running::Continue
            }
        }
    }
    async fn handle_reservation(
        &mut self,
        count: usize,
    ) -> Result<Vec<ReservationToken>, job::ReservationError> {
        let result = (0..count)
            .map(|_| {
                self.reservation_count += 1;
                ReservationToken {}
            })
            .collect();
        while self.actual_instance_count < self.desired_instance_count() {
            self.actual_instance_count += 1;
            let instance_rx = self.instance_rx.clone();
            tokio::spawn(async move {
                Instance::new(instance_rx).await.run().await;
            });
        }
        Ok(result)
    }
    async fn handle_execution(
        &mut self,
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        let (tx, rx) = oneshot::channel();
        self.instance_tx
            .send(InstanceMessage::Execution {
                dependencies,
                respond_to: tx,
            })
            .await
            .map_err(|e| job::ExecutionError::InternalError(format!("SendError: {e}")))?;
        let result = rx
            .await
            .map_err(|e| job::ExecutionError::InternalError(format!("RecvError: {e}")))?;

        drop(reservation);
        self.reservation_count -= 1;
        while self.actual_instance_count > self.desired_instance_count() {
            self.actual_instance_count -= 1;
            let (tx, rx) = oneshot::channel();
            self.instance_tx
                .send(InstanceMessage::Terminate { respond_to: tx })
                .await
                .map_err(|e| job::ExecutionError::InternalError(format!("SendError: {e}")))?;
            rx.await
                .map_err(|e| job::ExecutionError::InternalError(format!("RecvError: {e}")))?
                .map_err(|e| job::ExecutionError::InternalError(format!(": {e}")))?;
        }
        result
    }
    fn desired_instance_count(&self) -> usize {
        // TODO: consider this function
        self.reservation_count
    }
}
