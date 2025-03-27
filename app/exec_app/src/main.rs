use bollard::container::{Config, CreateContainerOptions};
use bollard::Docker;
use flate2::read::GzDecoder;
use judge_core::constant::env_var_exec::SCRIPT_PATH;
use judge_exec_grpc::generated::execute_service_server::{ExecuteService, ExecuteServiceServer};
use judge_exec_grpc::generated::{ExecuteRequest, ExecuteResponse};
use std::env;
use tar::Archive;
use tonic::async_trait;
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
        println!("{}", request.exec_time_ms);
        request.dependency.iter().for_each(|dep| {
            let tar = GzDecoder::new(&dep.outcome[..]);
            let mut archive = Archive::new(tar);
            // todo: error handling
            archive.unpack(format!("/outcome/{}", dep.envvar)).unwrap();
        });
        // todo: error handling
        let docker_api = Docker::connect_with_socket_defaults().unwrap();
        // todo: error handling
        let exec_container_entry_point = env::var(SCRIPT_PATH).unwrap();
        let env_vars: Vec<String> = request
            .dependency
            .iter()
            .map(|dep| format!("{}=\"/outcome/{}\"", dep.envvar, dep.envvar))
            .collect();
        let create_container_response = docker_api
            .create_container(
                Some(CreateContainerOptions {
                    name: ExecApp::DOCKER_CONTAINER_NAME,
                    platform: None,
                }),
                Config {
                    image: Some(ExecApp::DOCKER_IMAGE_NAME),
                    env: Some(env_vars.iter().map(|s| s.as_str()).collect()),
                    cmd: Some(vec![exec_container_entry_point.as_str()]),
                    ..Default::default()
                },
            )
            .await
            .unwrap(); // todo: error handling
        Ok(Response::new(ExecuteResponse {
            output: None,
            outcome: vec![],
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
