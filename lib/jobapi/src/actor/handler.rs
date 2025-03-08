#![allow(unused)]
use actix::prelude::*;
use judge_core::*;

use crate::{
    actor::{message::*, FileFactory, Instance, InstanceSupervisor},
    jobapi::{OutcomeToken, ReservationToken},
};

impl Handler<Reservation> for InstanceSupervisor {
    type Result = Result<Vec<ReservationToken>, job::ReservationError>;
    fn handle(&mut self, msg: Reservation, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<Execution> for InstanceSupervisor {
    type Result = Result<(OutcomeToken, std::process::Output), job::ExecutionError>;
    fn handle(&mut self, msg: Execution, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<Dependency> for Instance {
    type Result = Result<std::process::Output, job::ExecutionError>;
    fn handle(&mut self, msg: Dependency, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<FilePlacement> for FileFactory {
    type Result = Result<OutcomeToken, job::FilePlacementError>;
    fn handle(&mut self, msg: FilePlacement, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
