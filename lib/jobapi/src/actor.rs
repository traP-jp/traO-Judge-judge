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

pub struct Instance;
impl Actor for Instance {
    type Context = Context<Self>;
}

pub struct FileFactory;
impl Actor for FileFactory {
    type Context = Context<Self>;
}
