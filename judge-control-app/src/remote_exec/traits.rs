use std::time::Duration;
use tokio::net::ToSocketAddrs;
use std::error::Error;

pub trait RemoteExec<AddrType: ToSocketAddrs, ErrorType: Error> {
    async fn exec(
        &mut self,
        cmd: &str,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<String, ErrorType>;
}
