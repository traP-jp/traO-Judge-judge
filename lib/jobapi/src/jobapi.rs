#![allow(unused)]
use judge_core::model::*;

use crate::actor::{file_factory::FileFactory, instance_pool::InstancePool};

pub struct JobApi {
    instance_pool: InstancePool,
    file_factory: FileFactory,
}

impl JobApi {
    pub fn new() -> Self {
        todo!();
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
        todo!()
    }

    async fn execute(
        &self,
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> Result<(OutcomeToken, std::process::Output), job::ExecutionError> {
        todo!()
    }

    async fn place_file(
        &self,
        file_conf: job::FileConf,
    ) -> Result<OutcomeToken, job::FilePlacementError> {
        todo!()
    }
}
