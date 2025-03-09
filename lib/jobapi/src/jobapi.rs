#![allow(unused)]
use actix::prelude::*;
use judge_core::model::*;

use crate::actor::{message::*, FileFactory, InstanceSupervisor};

#[derive(Debug, Clone)]
pub struct JobApi {
    instance_supervisor: Addr<InstanceSupervisor>,
    file_factory: Addr<FileFactory>,
}

impl JobApi {
    fn new() -> Self {
        let instance_supervisor = InstanceSupervisor::default().start();
        let file_factory = FileFactory::new().start();
        Self {
            instance_supervisor,
            file_factory,
        }
    }
}

#[derive(Debug)]
pub struct ReservationToken {}

#[derive(Debug, Clone)]
pub struct OutcomeToken {}

#[axum::async_trait]
impl job::JobApi<ReservationToken, OutcomeToken> for JobApi {
    async fn reserve_execution(
        &self,
        count: usize,
    ) -> Result<Vec<ReservationToken>, job::ReservationError> {
        self.instance_supervisor
            .send(Reservation::new(count))
            .await
            .map_err(|e| job::ReservationError::ReserveFailed(format!("MailboxError: {e}")))?
    }

    async fn execute(
        &self,
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        self.instance_supervisor
            .send(Execution::new(reservation, dependencies))
            .await
            .map_err(|e| job::ExecutionError::InternalError(format!("MailboxError: {e}")))?
    }

    async fn place_file(
        &self,
        file_conf: job::FileConf,
    ) -> Result<OutcomeToken, job::FilePlacementError> {
        self.file_factory
            .send(FilePlacement::new(file_conf))
            .await
            .map_err(|e| job::FilePlacementError::PlaceFailed((format!("MailboxError: {e}"))))?
    }
}
