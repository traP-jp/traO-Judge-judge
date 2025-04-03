use judge_core::model::*;
use tokio::sync::oneshot;

use crate::actor::Running;
use crate::jobapi::OutcomeToken;

pub enum InstanceMessage {
    Execution {
        dependencies: Vec<job::Dependency<OutcomeToken>>,
        respond_to:
            oneshot::Sender<Result<(OutcomeToken, std::process::Output), job::ExecutionError>>,
    },
    Terminate {
        respond_to: oneshot::Sender<Result<(), anyhow::Error>>,
    },
}

pub struct Instance {
    // multi-consumer
    receiver: async_channel::Receiver<InstanceMessage>,
}

impl Instance {
    pub fn new(receiver: async_channel::Receiver<InstanceMessage>) -> Self {
        Self { receiver }
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
                dependencies,
                respond_to,
            } => {
                todo!();
                Running::Continue
            }
            InstanceMessage::Terminate { respond_to } => {
                todo!();
                Running::Stop
            }
        }
    }
}
