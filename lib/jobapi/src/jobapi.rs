use std::{future::Future, net::Ipv4Addr, path::PathBuf, sync::Arc};

use flate2::{Compression, write::GzEncoder};
use judge_core::{
    constant::env_var_exec,
    model::{job, problem_registry::ProblemRegistryClient},
};
use tokio::{
    io::AsyncWriteExt,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::{
    actor::{
        file_factory::{FileFactory, FileFactoryMessage},
        instance_pool::{InstancePool, InstancePoolMessage},
    },
    model::{aws::AwsClient, grpc::GrpcClient},
};

#[derive(Clone)]
pub struct JobApi {
    inner: Arc<JobApiInner>,
}

struct JobApiInner {
    instance_pool_tx: mpsc::UnboundedSender<InstancePoolMessage>,
    file_factory_tx: mpsc::UnboundedSender<FileFactoryMessage>,
}

impl JobApi {
    pub fn new<A, G, P, AFut, GFut, PFut, AF, GF, PF>(
        aws_client_factory: AF,
        grpc_client_factory: GF,
        problem_registry_client_factory: PF,
    ) -> Self
    where
        A: AwsClient + Send,
        G: GrpcClient + Send,
        P: ProblemRegistryClient + Send,
        AFut: Future<Output = A> + Send,
        GFut: Future<Output = G> + Send,
        PFut: Future<Output = P> + Send,
        AF: Fn() -> AFut + Send + Sync + Clone + 'static,
        GF: Fn(Ipv4Addr) -> GFut + Send + Sync + Clone + 'static,
        PF: Fn() -> PFut + Send + Sync + Clone + 'static,
    {
        let (instance_pool_tx, instance_pool_rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            InstancePool::new(instance_pool_rx, aws_client_factory, grpc_client_factory)
                .await
                .run()
                .await;
        });
        let (file_factory_tx, file_factory_rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            FileFactory::new(file_factory_rx, problem_registry_client_factory)
                .await
                .run()
                .await;
        });
        Self {
            inner: Arc::new(JobApiInner {
                instance_pool_tx,
                file_factory_tx,
            }),
        }
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
        self.inner
            .instance_pool_tx
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
        self.inner
            .instance_pool_tx
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
        self.inner
            .file_factory_tx
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
