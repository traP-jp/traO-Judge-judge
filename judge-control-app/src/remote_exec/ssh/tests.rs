#![allow(clippy::unwrap_used)]
use super::super::{ssh::SshConnection, traits::RemoteExec};
use anyhow::Result;
use std::time::Duration;
use uuid::Uuid;

#[tokio::test]
async fn test_ssh_connection() {
    let uuid = Uuid::new_v4();
    activate_container(uuid).await;
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

#[tokio::test]
async fn test_ssh_connection_timeout() {
    let uuid = Uuid::new_v4();
    activate_container(uuid).await;
    let mut ssh = SshConnection {
        addrs: "localhost:2022",
        username: "root".to_string(),
        password: "password".to_string(),
    };
    let resp = ssh
        .exec("sleep 1", Duration::from_millis(1), Duration::from_millis(1))
        .await;
    stop_ssh_docker_container(uuid).await.unwrap();
    assert!(resp.is_err_and(|e| match e {
        super::SshExecError::RemoteServerError(_) => true,
        _ => false,
    }));
    return;
}

async fn activate_container(uuid: Uuid) -> () {
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
