use judge_core::model::job;
use judge_core::model::problem_registry::ProblemRegistryClient as _;
use problem_registry::client::ProblemRegistryClient;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

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
    problem_registry_client: ProblemRegistryClient,
}

impl FileFactory {
    pub async fn new(receiver: mpsc::UnboundedReceiver<FileFactoryMessage>) -> Self {
        // create outcomes folder
        if let Err(e) = tokio::fs::create_dir("outcomes").await {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                _ => panic!("Something went wrong on create_dir")
            }
        }
        // warm-up ProblemRegistry client
        let problem_registry_client = ProblemRegistryClient::new().await;
        Self {
            receiver,
            problem_registry_client,
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
    async fn handle(&mut self, msg: FileFactoryMessage) -> Running {
        match msg {
            FileFactoryMessage::FilePlacement {
                file_conf,
                respond_to,
            } => {
                let result = self.handle_file_placement(file_conf).await;
                respond_to.send(result).unwrap();
                Running::Continue
            }
        }
    }
    async fn handle_file_placement(
        &mut self,
        file_conf: job::FileConf,
    ) -> Result<OutcomeToken, job::FilePlacementError> {
        let outcome_id = Uuid::now_v7();
        match file_conf {
            job::FileConf::EmptyDirectory => Ok(OutcomeToken::from_directory(outcome_id).await),
            job::FileConf::RuntimeText(content) => {
                Ok(OutcomeToken::from_text(outcome_id, content).await)
            }
            job::FileConf::Text(resource_id) => {
                let content = self
                    .problem_registry_client
                    .fetch(resource_id)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to fetch resource: {e}");
                        job::FilePlacementError::PlaceFailed(format!("ResourceFetchError: {e}"))
                    })?;
                Ok(OutcomeToken::from_text(outcome_id, content).await)
            }
        }
    }
}
