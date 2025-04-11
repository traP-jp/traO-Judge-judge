use anyhow::Context as _;
use bollard::container::{
    Config, CreateContainerOptions, DownloadFromContainerOptions, ListContainersOptions, LogOutput,
    RemoveContainerOptions, StartContainerOptions, UploadToContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::models::HostConfig;
use bollard::Docker;
use bytes::Bytes;
use flate2::read::GzDecoder;
use judge_core::constant::env_var_exec::{OUTPUT_PATH, SCRIPT_PATH};
use judge_exec_grpc::generated::execute_service_server::{ExecuteService, ExecuteServiceServer};
use judge_exec_grpc::generated::{Dependency, ExecuteRequest, ExecuteResponse, Output};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::ops::Not;
use tokio::time::timeout;
use tonic::async_trait;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

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
        let hashes: HashMap<_, _> = dependency
            .iter()
            .map(|dep| {
                (
                    dep.envvar.clone(),
                    format!("{:x}", Sha256::digest(&dep.outcome)),
                )
            })
            .map(|(envvar, hash)| {
                (
                    envvar.clone(),
                    format!("{:x}", Sha256::digest(hash + ":" + &envvar)),
                )
            })
            .collect();

        // create container
        let env_vars: Vec<String> = dependency
            .iter()
            .map(|dep| format!("{}=\"/outcome/{}\"", &dep.envvar, hashes[&dep.envvar]))
            .collect();
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
                println!("warning: {}", warning);
            });
        self.docker_api
            .start_container(
                ExecApp::DOCKER_CONTAINER_NAME,
                None::<StartContainerOptions<String>>,
            )
            .await?;

        // write outcomes to /outcome
        for dep in &dependency {
            let mut tar = GzDecoder::new(&dep.outcome[..]);
            let mut file: Vec<u8> = Vec::new();
            tar.read_to_end(file.as_mut())?;
            self.docker_api
                .upload_to_container(
                    ExecApp::DOCKER_CONTAINER_NAME,
                    Some(UploadToContainerOptions {
                        path: format!("/outcome/{}", hashes[&dep.envvar]),
                        no_overwrite_dir_non_dir: "True".parse()?,
                    }),
                    Bytes::from(file),
                )
                .await?;
        }

        // exec script
        let exec_container_entry_point = env::var(SCRIPT_PATH)?;
        self.docker_api
            .create_exec(
                ExecApp::DOCKER_CONTAINER_NAME,
                CreateExecOptions {
                    cmd: Some(vec!["chmod", "+x", exec_container_entry_point.as_str()]),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    ..CreateExecOptions::default()
                },
            )
            .await?;
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
                path: env::var(OUTPUT_PATH)?,
            }),
        );
        let mut output_bytes: Vec<u8> = vec![];
        while let Some(Ok(chunk)) = ouput.next().await {
            output_bytes.extend_from_slice(&chunk);
        }

        Ok(ExecuteResponse {
            output: Some(Output {
                exit_code: info.exit_code.context("failed to parse exit code")? as i32,
                stdout,
                stderr,
            }),
            outcome: output_bytes,
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
    let addr = "0.0.0.0:50051".parse().unwrap();
    let exec_app = ExecApp::default();
    Server::builder()
        .add_service(ExecuteServiceServer::new(exec_app))
        .serve(addr)
        .await?;
    Ok(())
}
