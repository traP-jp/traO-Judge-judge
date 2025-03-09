#![allow(unused)]
use actix::prelude::*;
use judge_core::model::*;

use crate::{
    actor::{message::*, FileFactory, Instance, InstanceSupervisor},
    jobapi::{OutcomeToken, ReservationToken},
};

impl Handler<Reservation> for InstanceSupervisor {
    type Result = Result<Vec<ReservationToken>, job::ReservationError>;
    fn handle(&mut self, msg: Reservation, ctx: &mut Self::Context) -> Self::Result {
        let mut result = (0..msg.count)
            .map(|_| {
                self.reservation_count += 1;
                ReservationToken {}
            })
            .collect();
        while self.instance_addrs.len() < self.calculate_desired_instance_count() {
            let instance = Instance.start();
            self.instance_addrs.push(instance);
        }
        Ok(result)
    }
}

impl Handler<Execution> for InstanceSupervisor {
    type Result = ResponseFuture<Result<(OutcomeToken, std::process::Output), job::ExecutionError>>;
    fn handle(&mut self, msg: Execution, ctx: &mut Self::Context) -> Self::Result {
        let Execution {
            reservation,
            dependencies,
        } = msg;
        drop(reservation);
        self.reservation_count -= 1;
        while self.instance_addrs.len() > self.calculate_desired_instance_count() {
            self.drop_one_instance_addr();
        }
        let future = self
            .get_target_instance_addr()
            .send(Dependency::new(dependencies));
        Box::pin(async move {
            future
                .await
                .map_err(|e| job::ExecutionError::InternalError(format!("MailboxError: {e}")))?
        })
    }
}

impl Handler<Dependency> for Instance {
    type Result = Result<(OutcomeToken, std::process::Output), job::ExecutionError>;
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
