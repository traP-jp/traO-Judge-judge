use anyhow::Result;
use std::net::ToSocketAddrs;

pub trait Telnet {
    fn new<Addr: ToSocketAddrs>(addr: Addr) -> Self;
    fn exec(&mut self, cmd: &str) -> Result<String>;
}
