#![allow(unused)]
use actix::prelude::*;
use judge_core::*;

use crate::jobapi::{OutcomeToken, ReservationToken};

#[derive(Message)]
#[rtype("Result<Vec<ReservationToken>, job::ReservationError>")]
pub struct Reservation {
    count: usize,
}

#[derive(Message)]
#[rtype("Result<(OutcomeToken, std::process::Output), job::ExecutionError>")]
pub struct Execution {
    reservation: ReservationToken,
    dependencies: Vec<job::Dependency<OutcomeToken>>,
}

#[derive(Message)]
#[rtype("Result<std::process::Output, job::ExecutionError>")]
pub struct Dependency {
    dependencies: Vec<job::Dependency<OutcomeToken>>,
}

#[derive(Message)]
#[rtype("Result<OutcomeToken, job::FilePlacementError>")]
pub struct FilePlacement {
    file_conf: job::FileConf,
}
