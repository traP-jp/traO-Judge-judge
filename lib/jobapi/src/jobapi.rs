#![allow(unused)]
use judge_core::model::*;

#[derive(Clone)]
pub struct JobApi {}

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
