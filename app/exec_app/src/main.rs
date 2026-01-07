use anyhow::Context as _;
use bollard::Docker;
use bollard::container::{
    Config, CreateContainerOptions, DownloadFromContainerOptions, ListContainersOptions, LogOutput,
    RemoveContainerOptions, StartContainerOptions, UploadToContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::models::{HostConfig, Mount, MountTypeEnum};
use bytes::Bytes;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use judge_core::constant::env_var_exec::{OUTPUT_PATH, SCRIPT_PATH};
use judge_exec_grpc::generated::execute_service_server::{ExecuteService, ExecuteServiceServer};
use judge_exec_grpc::generated::{Dependency, ExecuteRequest, ExecuteResponse, Output};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::Permissions;
use std::io::Read;
use std::ops::Not;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;
use std::{env, fs};
use tar::Archive;
use tokio::signal;
use tokio::time::timeout;
use tonic::async_trait;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{Request, Response, Status, transport::Server};

#[derive(Clone)]
pub struct ExecApp {
    docker_api: Docker,
    docker_image_name: String,
}

impl Default for ExecApp {
    fn default() -> Self {
        for key in ["DOCKER_IMAGE_NAME"] {
            if env::var(key).is_err() {
                panic!("{} is not set", key);
            }
        }
        ExecApp {
            docker_api: Docker::connect_with_socket_defaults().unwrap(),
            docker_image_name: env::var("DOCKER_IMAGE_NAME").unwrap(),
        }
    }
}

impl ExecApp {
    const DOCKER_CONTAINER_NAME: &'static str = "exec-container";

    async fn executing(&self) -> bool {
        let containers = self
            .docker_api
            .list_containers(None::<ListContainersOptions<String>>)
            .await
            .unwrap();
        containers.iter().any(|container| {
            container
                .names
                .clone()
                .unwrap_or(vec![])
                .contains(&ExecApp::DOCKER_CONTAINER_NAME.to_string())
        })
    }

    async fn terminate_container(&self) {
        if self.executing().await.not() {
            return;
        }
        self.docker_api
            .remove_container(
                ExecApp::DOCKER_CONTAINER_NAME,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
    }

    async fn execute_container(
        &self,
        dependency: Vec<Dependency>,
    ) -> Result<ExecuteResponse, anyhow::Error> {
        println!("{:?}", dependency);
        tracing::info!("writing outcomes");
        // write outcomes to /outcomes (dir in host)
        if fs::exists("/outcomes")? {
            fs::remove_dir_all("/outcomes")?;
        }
        fs::create_dir("/outcomes")?;
        tracing::info!("directory created");
        for dep in &dependency {
            let tar = GzDecoder::new(&dep.outcome[..]);
            let mut archive = Archive::new(tar);
            archive.unpack("/outcomes/")?;
            tracing::info!("file created: {}", &dep.envvar);
        }

        // create container
        let mut env_vars: Vec<String> = dependency
            .iter()
            .map(|dep| format!("{}=/outcomes/{}", &dep.envvar, dep.outcome_uuid))
            .collect();
        tracing::info!("env_vars: {:?}", env_vars);
        terminate_container(&self).await;
        tracing::info!("creating container");
        let create_container_response = self
            .docker_api
            .create_container(
                Some(CreateContainerOptions {
                    name: ExecApp::DOCKER_CONTAINER_NAME,
                    ..CreateContainerOptions::default()
                }),
                Config {
                    image: Some(self.docker_image_name.as_str()),
                    env: Some(env_vars.iter().map(|s| s.as_str()).collect()),
                    cmd: Some(vec!["sleep", "infinity"]),
                    host_config: Some(HostConfig {
                        cpuset_cpus: Some("0".to_string()),
                        memory: Some(2 * 1024 * 1024 * 1024), // 2GiB
                        mounts: Some(vec![Mount {
                            target: Some("/outcomes".to_string()),
                            source: Some("/outcomes".to_string()),
                            typ: Some(MountTypeEnum::BIND),
                            read_only: Some(false),
                            ..Default::default()
                        }]),
                        ..HostConfig::default()
                    }),
                    network_disabled: Some(true),
                    ..Default::default()
                },
            )
            .await?;
        create_container_response
            .warnings
            .iter()
            .for_each(|warning| {
                tracing::info!("warning: {}", warning);
            });
        tracing::info!("starting container");
        self.docker_api
            .start_container(
                ExecApp::DOCKER_CONTAINER_NAME,
                None::<StartContainerOptions<String>>,
            )
            .await?;

        tracing::info!("executing container");

        // exec script
        let exec_container_entry_point: String = dependency
            .iter()
            .filter(|dep| dep.envvar == SCRIPT_PATH)
            .map(|dep| format!("/outcomes/{}", dep.outcome_uuid))
            .next()
            .expect(format!("outcome \"{}\" not found", SCRIPT_PATH).as_str());

        tracing::info!("entrypoint: {}", exec_container_entry_point);

        fs::set_permissions(&exec_container_entry_point, Permissions::from_mode(0o755))?;
        tracing::info!("chmod done");
        let message = self
            .docker_api
            .create_exec(
                ExecApp::DOCKER_CONTAINER_NAME,
                CreateExecOptions {
                    cmd: Some(vec![exec_container_entry_point.as_str()]),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    ..CreateExecOptions::default()
                },
            )
            .await?;
        tracing::info!("exec created");

        tracing::info!("output: {:?}", message);

        // get exec result
        let result = self
            .docker_api
            .start_exec(&message.id, None::<StartExecOptions>)
            .await?;
        let mut stdout = String::new();
        let mut stderr = String::new();
        match result {
            StartExecResults::Attached { mut output, .. } => {
                while let Some(Ok(msg)) = output.next().await {
                    match msg {
                        LogOutput::StdErr { message } => {
                            let str = String::from_utf8_lossy(&message);
                            println!("stderr: {}", str); // todo
                            stderr.push_str(&str);
                        }
                        LogOutput::StdOut { message } => {
                            let str = String::from_utf8_lossy(&message);
                            println!("stdout: {}", str); // todo
                            stdout.push_str(&str);
                        }
                        _default => {}
                    }
                }
            }
            StartExecResults::Detached => {
                println!("Detached");
            }
        }

        // get exec info
        let info = self.docker_api.inspect_exec(&message.id).await?;

        let mut ouput = self.docker_api.download_from_container(
            ExecApp::DOCKER_CONTAINER_NAME,
            Some(DownloadFromContainerOptions::<String> {
                // TODO OUTPUT_PATHを実行時に渡す
                path: dependency
                    .iter()
                    .filter(|dep| dep.envvar == OUTPUT_PATH)
                    .map(|dep| format!("/outcomes/{}", dep.outcome_uuid))
                    .next()
                    .expect(format!("outcome \"{}\" not found", OUTPUT_PATH).as_str()),
            }),
        );

        let mut tar_bytes: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = ouput.next().await {
            tar_bytes.extend_from_slice(&chunk);
        }

        let mut gz_bytes = vec![];
        let mut encoder = GzEncoder::new(&mut gz_bytes, Compression::default());
        std::io::Write::write_all(&mut encoder, &tar_bytes)?;
        encoder.finish()?;

        Ok(ExecuteResponse {
            output: Some(Output {
                exit_code: info.exit_code.context("failed to parse exit code")? as i32,
                stdout,
                stderr,
            }),
            outcome: gz_bytes,
        })
    }
}

#[async_trait]
impl ExecuteService for ExecApp {
    async fn execute(
        &self,
        request: Request<ExecuteRequest>,
    ) -> Result<Response<ExecuteResponse>, Status> {
        if self.executing().await {
            return Ok(Response::new(ExecuteResponse {
                output: Some(Output {
                    exit_code: 1,
                    stdout: "judging".to_string(),
                    stderr: "".to_string(),
                }),
                outcome: vec![],
            }));
        }
        let request = request.into_inner();
        let exec_result = timeout(
            std::time::Duration::from_millis(request.exec_time_ms as u64),
            self.execute_container(request.dependency),
        );
        self.terminate_container().await;
        Ok(Response::new(match exec_result.await {
            Ok(Ok(response)) => response,
            Ok(Err(e)) => ExecuteResponse {
                // todo
                output: Some(Output {
                    exit_code: 1,
                    stdout: "error".to_string(),
                    stderr: e.to_string(),
                }),
                outcome: vec![],
            },
            Err(elapsed) => ExecuteResponse {
                // todo
                output: Some(Output {
                    exit_code: 1,
                    stdout: "timeout".to_string(),
                    stderr: elapsed.to_string(),
                }),
                outcome: vec![],
            },
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!("starting exec app");

    let addr = "0.0.0.0:50051".parse().unwrap();
    let exec_app = Arc::new(ExecApp::default());

    tracing::info!("cleaning up existing containers");
    exec_app.terminate_container().await;

    let exec_app_clone = exec_app.clone();

    let exec_app_panic = exec_app.clone();
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("panic occurred, cleaning up container");
        let exec_app = exec_app_panic.clone();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                exec_app.terminate_container().await;
            });
        });
        default_panic(info);
    }));

    let shutdown_handler = async move {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
        tracing::info!("shutdown signal received, cleaning up container");
        exec_app_clone.terminate_container().await;
        tracing::info!("container cleanup completed");
    };

    tokio::select! {
        result = Server::builder()
            .add_service(ExecuteServiceServer::new(exec_app.as_ref().clone()))
            .serve(addr) => {
            result?;
        }
        _ = shutdown_handler => {}
    }

    Ok(())
}
