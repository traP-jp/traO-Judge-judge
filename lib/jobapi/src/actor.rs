#![allow(unused)]
use actix::prelude::*;
use judge_core::*;

pub mod handler;
pub mod message;

pub struct InstanceSupervisor;
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
