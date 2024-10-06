#[cfg(test)]
mod tests;

use super::traits::RemoteExec;
use anyhow::{Context, Result};
use ssh2::{Channel, Session};
use std::io::Read;
use std::time::Duration;
use thiserror::Error as ThisError;
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};

pub struct SshConnection<AddrType: ToSocketAddrs> {
    pub addrs: AddrType,
    pub username: String,
    pub password: String,
}

#[derive(ThisError, Debug)]
pub enum SshExecError {
    #[error("Execution in remote SSH server failed")]
    RemoteExecutionFailed(#[from] RemoteServerError),
    #[error("Internal error while SSH execution")]
    InternalServerError(#[from] InternalServerError),
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct RemoteServerError(anyhow::Error);

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct InternalServerError(anyhow::Error);

impl<AddrType: ToSocketAddrs> RemoteExec<AddrType, SshExecError> for SshConnection<AddrType> {
    async fn exec(
        &mut self,
        cmd: &str,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<String, SshExecError> {
        let channel = self
            .connect_with_timeout(connection_time_limit)
            .await
            .map_err(|e| SshExecError::InternalServerError(e))?;
        let output = self
            .exec_inner_with_timeout(cmd, channel, execution_time_limit)
            .await
            .map_err(|e| SshExecError::RemoteExecutionFailed(e))?;
        Ok(output)
    }
}

impl<AddrType: ToSocketAddrs> SshConnection<AddrType> {
    // SSH接続の確立
    async fn connect_with_timeout(
        &self,
        connection_time_limit: Duration,
    ) -> Result<Channel, InternalServerError> {
        let connect_future = async move {
            let tcp = TcpStream::connect(&self.addrs)
                .await
                .context("Failed to connect to the SSH server")?;
            let mut sess = Session::new().context("Failed to create a new SSH session")?;
            sess.set_tcp_stream(tcp);
            sess.handshake()
                .context("Failed to perform SSH handshake")?;
            sess.userauth_password(&self.username, &self.password)
                .context("Failed to authenticate with the SSH server")?;
            let chan = sess
                .channel_session()
                .context("Failed to open a new channel for SSH")?;
            Ok(chan)
        };
        let timeout_future = async move {
            let result = timeout(connection_time_limit, connect_future)
                .await
                .map_err(|e| anyhow::Error::from(e))
                .context("Connection time limit exceeded")?;
            result
        };
        let result: Result<Channel, InternalServerError> = timeout_future
            .await
            .map_err(|e| InternalServerError(e).into());
        result
    }

    // コマンドの実行
    async fn exec_inner_with_timeout(
        &self,
        cmd: &str,
        mut chan: Channel,
        execution_time_limit: Duration,
    ) -> Result<String, RemoteServerError> {
        let exec_future = async move {
            chan.exec(cmd)
                .context("Failed to execute the command via SSH")?;
            let mut output = String::new();
            chan.read_to_string(&mut output)
                .context("Failed to read the output from SSH")?;
            Ok(output)
        };
        let timeout_future = async move {
            let result = timeout(execution_time_limit, exec_future)
                .await
                .map_err(|e| anyhow::Error::from(e))
                .context("Execution time limit exceeded")?;
            result
        };
        let result: Result<String, RemoteServerError> = timeout_future
            .await
            .map_err(|e| RemoteServerError(e).into());
        result
    }
}
