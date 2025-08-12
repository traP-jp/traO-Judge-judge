use back_judge_grpc::{
    generated::judge_service_server::JudgeServiceServer, server::WrappedJudgeService,
};
use job_service::{aws::AwsClient, grpc::GrpcClient, job_service::JobService};
use judge_core::logic::judge_service_impl::JudgeServiceImpl;
use problem_registry::client::ProblemRegistryClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let grpc_service_port = std::env::var("TRAOJUDGE_GRPC_SERVICE_PORT")
        .expect("TRAOJUDGE_GRPC_SERVICE_PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse TRAOJUDGE_GRPC_SERVICE_PORT");
    let grpc_service_addr = format!("0.0.0.0:{}", grpc_service_port)
        .parse::<std::net::SocketAddr>()
        .expect("Failed to parse grpc service address");
    let aws_client_factory = || async move { AwsClient::new().await };
    let grpc_client_factory = |ip_addr| async move { GrpcClient::new(ip_addr).await };
    let problem_registry_client_factory = || async move { ProblemRegistryClient::new().await };
    let job_service = JobService::new(
        aws_client_factory,
        grpc_client_factory,
        problem_registry_client_factory,
    );
    tracing::info!("JobService created");
    let inner_judge_service = JudgeServiceImpl::new(job_service);
    tracing::info!("JudgeServiceImpl created");
    let wrapped_judge_service = WrappedJudgeService::new(inner_judge_service);
    let grpc_service = JudgeServiceServer::new(wrapped_judge_service);
    tracing::info!("JudgeServiceServer created");
    tracing::info!("Starting grpc service on {}", grpc_service_addr);
    tonic::transport::Server::builder()
        .add_service(grpc_service)
        .serve(grpc_service_addr)
        .await
        .expect("Failed to serve grpc service");
}
