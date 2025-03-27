use anyhow::Context as _;
use bollard::container::{Config, CreateContainerOptions, LogOutput};
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::models::HostConfig;
use bollard::Docker;
use flate2::read::GzDecoder;
use judge_core::constant::env_var_exec::SCRIPT_PATH;
use judge_exec_grpc::generated::execute_service_server::{ExecuteService, ExecuteServiceServer};
use judge_exec_grpc::generated::{Dependency, ExecuteRequest, ExecuteResponse, Output};
use std::env;
use tar::Archive;
use tokio::time::timeout;
use tonic::async_trait;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct ExecApp {}

impl ExecApp {
    const DOCKER_IMAGE_NAME: &'static str = "exec-container-image";
    const DOCKER_CONTAINER_NAME: &'static str = "exec-container";

    async fn execute_container(
        &self,
        dependency: Vec<Dependency>,
    ) -> Result<ExecuteResponse, anyhow::Error> {
        // write outcomes to /outcome
        // todo: note: we can use Docker::upload_to_container instead of writing to disk
        dependency.iter().try_for_each(|dep| {
            let tar = GzDecoder::new(&dep.outcome[..]);
            let mut archive = Archive::new(tar);
            archive.unpack(format!("/outcome/{}", dep.envvar))
        })?;

        // connect to docker
        let docker_api = Docker::connect_with_socket_defaults()?;

        // create container
        let env_vars: Vec<String> = dependency
            .iter()
            .map(|dep| format!("{}=\"/outcome/{}\"", dep.envvar, dep.envvar))
            .collect();
        let create_container_response = docker_api
            .create_container(
                Some(CreateContainerOptions {
                    name: ExecApp::DOCKER_CONTAINER_NAME,
                    ..CreateContainerOptions::default()
                }),
                Config {
                    image: Some(ExecApp::DOCKER_IMAGE_NAME),
                    env: Some(env_vars.iter().map(|s| s.as_str()).collect()),
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

        // exec script
        let exec_container_entry_point = env::var(SCRIPT_PATH)?;
        let message = docker_api
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
        let result = docker_api
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
        let info = docker_api.inspect_exec(&message.id).await?;

        Ok(ExecuteResponse {
            output: Some(Output {
                exit_code: info.exit_code.context("failed to parse exit code")? as i32,
                stdout,
                stderr,
            }),
            outcome: vec![], // TODO
        })
    }
}

#[async_trait]
impl ExecuteService for ExecApp {
    async fn execute(
        &self,
        request: Request<ExecuteRequest>,
    ) -> Result<Response<ExecuteResponse>, Status> {
        let request = request.into_inner();
        let exec_result = timeout(
            std::time::Duration::from_millis(request.exec_time_ms as u64),
            self.execute_container(request.dependency),
        );
        Ok(Response::new(match exec_result.await {
            Ok(Ok(response)) => response,
            Ok(Err(e)) => ExecuteResponse {
                // todo
                output: Some(Output {
                    exit_code: -1,
                    stdout: "error".to_string(),
                    stderr: e.to_string(),
                }),
                outcome: vec![],
            },
            Err(elapsed) => ExecuteResponse {
                // todo
                output: Some(Output {
                    exit_code: -1,
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
