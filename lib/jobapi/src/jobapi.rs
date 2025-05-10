use std::path::PathBuf;

use flate2::{write::GzEncoder, Compression};
use judge_core::{constant::env_var_exec, model::job};
use tokio::{
    io::AsyncWriteExt,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::actor::{
    file_factory::{FileFactory, FileFactoryMessage},
    instance_pool::{InstancePool, InstancePoolMessage},
};

pub struct JobApi {
    instance_pool_tx: mpsc::UnboundedSender<InstancePoolMessage>,
    file_factory_tx: mpsc::UnboundedSender<FileFactoryMessage>,
}

impl JobApi {
    pub fn new() -> Self {
        let (instance_pool_tx, instance_pool_rx) = mpsc::unbounded_channel();
        // TODO: join
        tokio::spawn(async move {
            InstancePool::new(instance_pool_rx).await.run().await;
        });
        let (file_factory_tx, file_factory_rx) = mpsc::unbounded_channel();
        // TODO: join
        tokio::spawn(async move {
            FileFactory::new(file_factory_rx).await.run().await;
        });
        Self {
            instance_pool_tx,
            file_factory_tx,
        }
    }
}

impl Clone for JobApi {
    fn clone(&self) -> Self {
        JobApi::new()
    }
}

#[derive(Debug)]
pub struct ReservationToken {}

#[derive(Debug, Clone)]
pub struct OutcomeToken {
    pub outcome_id: Uuid,
    path_to_tar_gz: PathBuf,
}

impl OutcomeToken {
    // TODO: avoid unwrap
    pub async fn from_directory(outcome_id: Uuid) -> Self {
        let mut tar_buf = vec![];
        let enc = GzEncoder::new(&mut tar_buf, Compression::default());
        let mut tar = tar::Builder::new(enc);
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(tar::EntryType::Directory);
        header.set_size(0);
        header.set_cksum();
        let dir_name = format!("{}/", outcome_id);
        tar.append_data(&mut header, dir_name, std::io::empty())
            .unwrap();
        tar.finish().unwrap();
        let enc = tar.into_inner().unwrap();
        enc.finish().unwrap();
        OutcomeToken::from_binary(outcome_id, &tar_buf).await
    }
    pub async fn from_text(outcome_id: Uuid, text: String) -> Self {
        let mut tar_buf = vec![];
        let enc = GzEncoder::new(&mut tar_buf, Compression::default());
        let mut tar = tar::Builder::new(enc);
        let mut header = tar::Header::new_gnu();
        header.set_size(text.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        let file_name = format!("{}", outcome_id);
        tar.append_data(&mut header, file_name, text.as_bytes())
            .unwrap();
        tar.finish().unwrap();
        let enc = tar.into_inner().unwrap();
        enc.finish().unwrap();
        OutcomeToken::from_binary(outcome_id, &tar_buf).await
    }
    pub async fn from_binary(outcome_id: Uuid, binary: &[u8]) -> Self {
        let path_to_tar_gz = PathBuf::from(format!("outcomes/{outcome_id}.tar.gz"));
        let mut file = tokio::fs::File::create(path_to_tar_gz.clone())
            .await
            .unwrap();
        file.write_all(binary).await.unwrap();
        Self {
            outcome_id,
            path_to_tar_gz,
        }
    }
    pub async fn to_binary(&self) -> Vec<u8> {
        tokio::fs::read(self.path_to_tar_gz.clone()).await.unwrap()
    }
}

#[axum::async_trait]
impl job::JobApi<ReservationToken, OutcomeToken> for JobApi {
    async fn reserve_execution(
        &self,
        count: usize,
    ) -> Result<Vec<ReservationToken>, job::ReservationError> {
        let (tx, rx) = oneshot::channel();
        self.instance_pool_tx
            .send(InstancePoolMessage::Reservation {
                count,
                respond_to: tx,
            })
            .map_err(|e| {
                tracing::error!("Failed to send InstancePoolMessage::Reservation: {e}");
                job::ReservationError::ReserveFailed(format!("SendError: {e}"))
            })?;
        rx.await.map_err(|e| {
            tracing::error!("Failed to recv response of InstancePoolMessage::Reservation: {e}");
            job::ReservationError::ReserveFailed(format!("RecvError: {e}"))
        })?
    }

    async fn execute(
        &self,
        reservation: ReservationToken,
        mut dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        let outcome_for_res = self
            .place_file(job::FileConf::EmptyDirectory)
            .await
            .map_err(|e| {
                job::ExecutionError::InternalError(format!("Failed to create EmptyDirectory: {e}"))
            })?;
        let dependency_for_res = job::Dependency {
            envvar: env_var_exec::OUTPUT_PATH.to_string(),
            outcome: outcome_for_res.clone(),
        };
        dependencies.push(dependency_for_res);
        let (tx, rx) = oneshot::channel();
        self.instance_pool_tx
            .send(InstancePoolMessage::Execution {
                reservation,
                outcome_id_for_res: outcome_for_res.outcome_id,
                dependencies,
                respond_to: tx,
            })
            .map_err(|e| {
                tracing::error!("Failed to send InstancePoolMessage::Execution: {e}");
                job::ExecutionError::InternalError(format!("SendError: {e}"))
            })?;
        rx.await.map_err(|e| {
            tracing::error!("Failed to recv response of InstancePoolMessage::Execution: {e}");
            job::ExecutionError::InternalError(format!("RecvError: {e}"))
        })?
    }

    async fn place_file(
        &self,
        file_conf: job::FileConf,
    ) -> Result<OutcomeToken, job::FilePlacementError> {
        let (tx, rx) = oneshot::channel();
        self.file_factory_tx
            .send(FileFactoryMessage::FilePlacement {
                file_conf,
                respond_to: tx,
            })
            .map_err(|e| {
                tracing::error!("Failed to send FileFactoryMessage::FilePlacement: {e}");
                job::FilePlacementError::PlaceFailed(format!("SendError: {e}"))
            })?;
        rx.await.map_err(|e| {
            tracing::error!("Failed to recv response of FileFactoryMessage::FilePlacement: {e}");
            job::FilePlacementError::PlaceFailed(format!("RecvError: {e}"))
        })?
    }
}
