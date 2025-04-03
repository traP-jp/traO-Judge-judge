use judge_core::model::*;
use tokio::sync::{mpsc, oneshot};

use crate::actor::Running;
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
}

impl InstancePool {
    pub fn new(receiver: mpsc::UnboundedReceiver<InstancePoolMessage>) -> Self {
        Self { receiver }
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
                todo!();
                Running::Continue
            }
            InstancePoolMessage::Execution {
                reservation,
                dependencies,
                respond_to,
            } => {
                todo!();
                Running::Continue
            }
        }
    }
}
