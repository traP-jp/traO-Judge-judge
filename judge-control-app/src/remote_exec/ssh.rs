use super::traits::RemoteExec;
use anyhow::{Context, Result};
use ssh2::{Channel, Session};
use std::io::Read;
use std::time::Duration;
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};

pub struct SshConnection<AddrType: ToSocketAddrs> {
    addrs: AddrType,
    username: String,
    password: String,
}

impl<AddrType: ToSocketAddrs> RemoteExec<AddrType> for SshConnection<AddrType> {
    async fn exec(
        &mut self,
        cmd: &str,
        connection_time_limit: Duration,
        execution_time_limit: Duration,
    ) -> Result<String> {
        let connect_future = async move {
            // SSH接続の確立
            let channel = self
                .connect()
                .await?;
            // 時間制限付きでコマンドを実行
            let exec_with_timeout_future = timeout(
                execution_time_limit,
                self.exec_inner(cmd, channel)).await?;
            let result = exec_with_timeout_future.context("Execution time limit exceeded")?;
            Ok(result)
        };
        // 接続時間制限付きでSSH接続
        let connect_with_timeout_future = timeout(
            connection_time_limit,
            connect_future
        );
        let result = connect_with_timeout_future
            .await
            .context("Connection time limit exceeded")?;
        result
    }
}

impl<AddrType: ToSocketAddrs> SshConnection<AddrType> {
    // SSH接続の確立
    async fn connect(&self) -> Result<Channel> {
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
    }

    // コマンドの実行
    async fn exec_inner(&self, cmd: &str, mut chan: Channel) -> Result<String> {
        chan.exec(cmd).context("Failed to execute the command via SSH")?;
        let mut output = String::new();
        chan.read_to_string(&mut output).context("Failed to read the output from SSH")?;
        Ok(output)
    }
}
