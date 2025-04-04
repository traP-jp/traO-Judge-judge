use back_judge_grpc::{
    generated::judge_service_server::JudgeServiceServer, server::WrappedJudgeApi,
};
use jobapi::jobapi::JobApi;
use judge_core::logic::judge_api_impl::JudgeApiImpl;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let grpc_service_port = std::env::var("TRAOJUDGE_GRPC_SERVICE_PORT")
        .expect("TRAOJUDGE_GRPC_SERVICE_PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse TRAOJUDGE_GRPC_SERVICE_PORT");
    let grpc_service_addr = format!("127.0.0.1:{}", grpc_service_port)
        .parse::<std::net::SocketAddr>()
        .expect("Failed to parse grpc service address");
    tracing::info!("ProblemRegistryClient created");
    let jobapi = JobApi::new();
    tracing::info!("JobApi created");
    let inner_judge_api = JudgeApiImpl::new(jobapi);
    tracing::info!("JudgeApiImpl created");
    let wrapped_judge_api = WrappedJudgeApi::new(inner_judge_api);
    let grpc_service = JudgeServiceServer::new(wrapped_judge_api);
    tracing::info!("JudgeServiceServer created");
    tracing::info!("Starting grpc service on {}", grpc_service_addr);
    tonic::transport::Server::builder()
        .add_service(grpc_service)
        .serve(grpc_service_addr)
        .await
        .expect("Failed to serve grpc service");
}
