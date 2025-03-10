#![allow(unused)]
use actix::prelude::*;
use judge_core::*;

pub mod handler;
pub mod message;

const MAILBOX_CAPACITY: usize = usize::MAX;

#[derive(Default)]
pub struct InstanceSupervisor {
    reservation_count: usize,
    instance_addrs: Vec<Addr<Instance>>,
}

impl InstanceSupervisor {
    pub fn calculate_desired_instance_count(&self) -> usize {
        todo!("self.reservation_count から適切なインスタンス数 (0~15) を決定し、返す")
    }
    pub fn get_target_instance_addr(&self) -> Addr<Instance> {
        todo!("self.instance_addrs から実行を行うインスタンスを決定し、返す")
    }
    pub fn drop_one_instance_addr(&mut self) {
        todo!("self.instance_addrs から削除するインスタンスを決定し、drop する")
    }
}

impl Actor for InstanceSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(MAILBOX_CAPACITY);
    }
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
        ctx.set_mailbox_capacity(MAILBOX_CAPACITY);
        todo!("AwsClient の create_instance 呼び出し");
        todo!("self.instance_url 書き換え");
        todo!("exec の http サーバ起動待ち、定期的にポーリングして反応あったら終了");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        todo!("AwsClient の terminate_instance 呼び出し");
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

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(MAILBOX_CAPACITY);
    }
}
