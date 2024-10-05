use anyhow::Result;
use std::time::Duration;
use tokio::net::ToSocketAddrs;

pub trait RemoteExec<AddrType: ToSocketAddrs> {
    async fn exec(
        &mut self,
        cmd: &str,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<String>;
}
