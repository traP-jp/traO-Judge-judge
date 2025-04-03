use judge_core::model::*;
use tokio::sync::{mpsc, oneshot};

use crate::actor::Running;
use crate::jobapi::OutcomeToken;

pub enum FileFactoryMessage {
    FilePlacement {
        file_conf: job::FileConf,
        respond_to: oneshot::Sender<Result<OutcomeToken, job::FilePlacementError>>,
    },
}

pub struct FileFactory {
    receiver: mpsc::UnboundedReceiver<FileFactoryMessage>,
}

impl FileFactory {
    pub fn new(receiver: mpsc::UnboundedReceiver<FileFactoryMessage>) -> Self {
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
    async fn handle(&mut self, msg: FileFactoryMessage) -> Running {
        match msg {
            FileFactoryMessage::FilePlacement {
                file_conf,
                respond_to,
            } => {
                todo!();
                Running::Continue
            }
        }
    }
}
