use bollard::container::{Config, CreateContainerOptions, LogOutput};
use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use bollard::models::HostConfig;
use bollard::Docker;
use flate2::read::GzDecoder;
use judge_core::constant::env_var_exec::SCRIPT_PATH;
use judge_exec_grpc::generated::execute_service_server::{ExecuteService, ExecuteServiceServer};
use judge_exec_grpc::generated::{ExecuteRequest, ExecuteResponse, Output};
use std::env;
use tar::Archive;
use tonic::async_trait;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct ExecApp {}

impl ExecApp {
    const DOCKER_IMAGE_NAME: &'static str = "exec-container-image";
    const DOCKER_CONTAINER_NAME: &'static str = "exec-container";
}

#[async_trait]
impl ExecuteService for ExecApp {
    async fn execute(
        &self,
        request: Request<ExecuteRequest>,
    ) -> Result<Response<ExecuteResponse>, Status> {
        let request = request.into_inner();

        // write outcomes to /outcome
        // todo: note: we can use Docker::upload_to_container instead of writing to disk
        request.dependency.iter().for_each(|dep| {
            let tar = GzDecoder::new(&dep.outcome[..]);
            let mut archive = Archive::new(tar);
            // todo: error handling
            archive.unpack(format!("/outcome/{}", dep.envvar)).unwrap();
        });

        // connect to docker
        // todo: error handling
        let docker_api = Docker::connect_with_socket_defaults().unwrap();

        // create container
        // todo: error handling
        let env_vars: Vec<String> = request
            .dependency
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
                    // cmd: Some(vec![exec_container_entry_point.as_str()]),
                    host_config: Some(HostConfig {
                        cpuset_cpus: Some("0".to_string()),
                        memory: Some(2 * 1024 * 1024 * 1024), // 2GiB
                        ..HostConfig::default()
                    }),
                    network_disabled: Some(true),
                    ..Default::default()
                },
            )
            .await
            .unwrap(); // todo: error handling
        create_container_response
            .warnings
            .iter()
            .for_each(|warning| {
                println!("warning: {}", warning);
            });

        // exec script
        let exec_container_entry_point = env::var(SCRIPT_PATH).unwrap();
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
            .await
            .unwrap();

        // get exec result
        let result = docker_api
            .start_exec(&message.id, None::<StartExecOptions>)
            .await
            .unwrap(); // todo: error handling
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
        // todo: error handling
        let info = docker_api.inspect_exec(&message.id).await.unwrap();

        // let mut stream = docker_api.wait_container(
        //     ExecApp::DOCKER_CONTAINER_NAME,
        //     Some(WaitContainerOptions {
        //         condition: "not-running",
        //     }),
        // );
        // let exec_result = timeout(
        //     std::time::Duration::from_millis(request.exec_time_ms as u64),
        //     stream.next(),
        // )
        // .await
        // .unwrap()
        // .unwrap()
        // .unwrap(); // todo: error handling

        Ok(Response::new(ExecuteResponse {
            output: Some(Output {
                exit_code: info.exit_code.unwrap() as i32, // todo: error handling
                stdout,
                stderr,
            }),
            outcome: vec![], // TODO
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
