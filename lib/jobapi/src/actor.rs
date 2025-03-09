#![allow(unused)]
use actix::prelude::*;
use judge_core::*;

pub mod handler;
pub mod message;

#[derive(Default)]
pub struct InstanceSupervisor {
    reservation_count: usize,
    instance_addrs: Vec<Addr<Instance>>,
}

impl InstanceSupervisor {
    pub fn calculate_desired_instance_count(&self) -> usize {
        todo!()
    }
    pub fn get_target_instance_addr(&self) -> Addr<Instance> {
        todo!()
    }
    pub fn drop_one_instance_addr(&mut self) {
        todo!()
    }
}

impl Actor for InstanceSupervisor {
    type Context = Context<Self>;
}

pub struct Instance {
    instance_id: uuid::Uuid,
    instance_url: Option<std::net::Ipv4Addr>,
}

impl Instance {
    pub fn new(instance_id: uuid::Uuid) -> Self {
        Self {
            instance_id,
            instance_url: None,
        }
    }
}

impl Actor for Instance {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        todo!("AWS インスタンス作成処理");
        todo!("self.instance_url 書き換え");
        todo!("exec の http サーバ起動待ち、定期的にポーリングする");
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        todo!("AWS インスタンス削除処理")
    }
}

pub struct FileFactory {}

impl FileFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for FileFactory {
    type Context = Context<Self>;
}
