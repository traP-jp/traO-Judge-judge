use flate2::read::GzDecoder;
use judge_exec_grpc::generated::execute_service_server::{ExecuteService, ExecuteServiceServer};
use judge_exec_grpc::generated::{ExecuteRequest, ExecuteResponse};
use tar::Archive;
use tonic::async_trait;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct ExecApp {}

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
            archive.unpack("./outcome").unwrap();
            // TODO: Run the container and pass dep.outcome to it
        });
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
