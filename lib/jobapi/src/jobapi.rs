#![allow(unused)]
use judge_core::model::*;
use tokio::sync::{mpsc, oneshot};

use crate::actor::{
    file_factory::{FileFactory, FileFactoryMessage},
    instance_pool::{self, InstancePool, InstancePoolMessage},
};

pub struct JobApi {
    instance_pool_tx: mpsc::UnboundedSender<InstancePoolMessage>,
    file_factory_tx: mpsc::UnboundedSender<FileFactoryMessage>,
}

impl JobApi {
    pub fn new() -> Self {
        let (instance_pool_tx, instance_pool_rx) = mpsc::unbounded_channel();
        let mut instance_pool = InstancePool::new(instance_pool_rx);
        // TODO: join
        tokio::spawn(async move {
            instance_pool.run().await;
        });
        let (file_factory_tx, file_factory_rx) = mpsc::unbounded_channel();
        let mut file_factory = FileFactory::new(file_factory_rx);
        // TODO: join
        tokio::spawn(async move {
            file_factory.run().await;
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

pub struct ReservationToken {}

#[derive(Clone)]
pub struct OutcomeToken {}

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
            });
        rx.await
            .map_err(|e| job::ReservationError::ReserveFailed(format!("RecvError: {e}")))?
    }

    async fn execute(
        &self,
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        let (tx, rx) = oneshot::channel();
        self.instance_pool_tx.send(InstancePoolMessage::Execution {
            reservation,
            dependencies,
            respond_to: tx,
        });
        rx.await
            .map_err(|e| job::ExecutionError::InternalError(format!("RecvError: {e}")))?
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
            });
        rx.await
            .map_err(|e| job::FilePlacementError::PlaceFailed(format!("RecvError: {e}")))?
    }
}
