#![allow(unused)]
use actix::prelude::*;
use judge_core::model::*;

use crate::jobapi::{OutcomeToken, ReservationToken};

#[derive(Debug, Message)]
#[rtype("Result<Vec<ReservationToken>, job::ReservationError>")]
pub struct Reservation {
    pub count: usize,
}

impl Reservation {
    pub fn new(count: usize) -> Self {
        Self { count }
    }
}

#[derive(Debug, Message)]
#[rtype("Result<(OutcomeToken, std::process::Output), job::ExecutionError>")]
pub struct Execution {
    pub reservation: ReservationToken,
    pub dependencies: Vec<job::Dependency<OutcomeToken>>,
}

impl Execution {
    pub fn new(
        reservation: ReservationToken,
        dependencies: Vec<job::Dependency<OutcomeToken>>,
    ) -> Self {
        Self {
            reservation,
            dependencies,
        }
    }
}

#[derive(Debug, Message)]
#[rtype("Result<(OutcomeToken, std::process::Output), job::ExecutionError>")]
pub struct Dependency {
    dependencies: Vec<job::Dependency<OutcomeToken>>,
}

impl Dependency {
    pub fn new(dependencies: Vec<job::Dependency<OutcomeToken>>) -> Self {
        Self { dependencies }
    }
}

#[derive(Debug, Message)]
#[rtype("Result<OutcomeToken, job::FilePlacementError>")]
pub struct FilePlacement {
    pub file_conf: job::FileConf,
}

impl FilePlacement {
    pub fn new(file_conf: job::FileConf) -> Self {
        Self { file_conf }
    }
}
