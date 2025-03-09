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
            let instance_id = uuid::Uuid::now_v7();
            let instance = Instance::new(instance_id).start();
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
        todo!("OutcomeToken に対応するファイルを exec から見えるように配置");
        todo!("exec に http リクエスト送信");
        todo!("レスポンス (std::process::Output) 受信");
        todo!("成果物 (OutcomeToken) 取得");
    }
}

impl Handler<FilePlacement> for FileFactory {
    type Result = Result<OutcomeToken, job::FilePlacementError>;
    fn handle(&mut self, msg: FilePlacement, ctx: &mut Self::Context) -> Self::Result {
        match msg.file_conf {
            job::FileConf::EmptyDirectory => {
                todo!()
            }
            job::FileConf::Text(_) => {
                todo!()
            }
            job::FileConf::RuntimeText(_) => {
                todo!()
            }
        }
        todo!("配置したファイル (OutcomeToken) 取得")
    }
}
