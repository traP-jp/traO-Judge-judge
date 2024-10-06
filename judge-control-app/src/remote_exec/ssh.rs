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
            let channel = self.connect().await?;
            // 時間制限付きでコマンドを実行
            let exec_with_timeout_future =
                timeout(execution_time_limit, self.exec_inner(cmd, channel)).await?;
            let result = exec_with_timeout_future.context("Execution time limit exceeded")?;
            Ok(result)
        };
        // 接続時間制限付きでSSH接続
        let connect_with_timeout_future = timeout(connection_time_limit, connect_future);
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
        chan.exec(cmd)
            .context("Failed to execute the command via SSH")?;
        let mut output = String::new();
        chan.read_to_string(&mut output)
            .context("Failed to read the output from SSH")?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_ssh_connection() {
        let uuid = Uuid::new_v4();
        if !check_docker_installed().await {
            return;
        }
        if !check_docker_running().await {
            if check_su_privilege().await {
                start_docker_daemon().await.unwrap();
            } else {
                eprintln!("Neither docker is running nor you have su privilege");
                return;
            }
        }
        build_ssh_docker_image(uuid).await.unwrap();
        let result = run_ssh_docker_container(uuid).await;
        remove_docker_image(uuid).await.unwrap();
        assert!(result.is_ok());
        let mut ssh = SshConnection {
            addrs: "localhost:2022",
            username: "root".to_string(),
            password: "password".to_string(),
        };
        let resp = ssh
            .exec(
                "cat /flag",
                Duration::from_secs(60),
                Duration::from_secs(60),
            )
            .await;
        stop_ssh_docker_container(uuid).await.unwrap();
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap(), "TEST_FLAG\n");
    }

    async fn check_docker_installed() -> bool {
        let output = std::process::Command::new("docker")
            .arg("--version")
            .output()
            .unwrap();
        output.status.success()
    }

    async fn start_docker_daemon() -> Result<()> {
        let _ = std::process::Command::new("systemctl")
            .args(&["start", "docker"])
            .output()?;
        Ok(())
    }

    async fn check_docker_running() -> bool {
        let output = std::process::Command::new("systemctl")
            .args(&["is-active", "docker"])
            .output()
            .unwrap();
        output.status.success()
    }

    async fn check_su_privilege() -> bool {
        let output = std::process::Command::new("su")
            .arg("-c")
            .arg("whoami")
            .output()
            .unwrap();
        output.status.success()
    }

    async fn build_ssh_docker_image(uuid: Uuid) -> Result<()> {
        let _ = std::process::Command::new("docker")
            .args(&["build", "-t", &format!("ssh-server-test-{}", uuid), "."])
            .current_dir("tests/ssh_server")
            .output()?;
        Ok(())
    }

    async fn remove_docker_image(uuid: Uuid) -> Result<()> {
        let _ = std::process::Command::new("docker")
            .args(&["rmi", &format!("ssh-server-test-{}", uuid)])
            .output()?;
        Ok(())
    }

    async fn run_ssh_docker_container(uuid: Uuid) -> Result<()> {
        let _ = std::process::Command::new("docker")
            .args(&[
                "run",
                "-d",
                "-p",
                "2022:2022",
                "--name",
                &format!("ssh-server-test-{}", uuid),
                &format!("ssh-server-test-{}", uuid),
            ])
            .output()?;
        Ok(())
    }

    async fn stop_ssh_docker_container(uuid: Uuid) -> Result<()> {
        let _ = std::process::Command::new("docker")
            .args(&["stop", &format!("ssh-server-test-{}", uuid)])
            .output()?;
        Ok(())
    }
}
