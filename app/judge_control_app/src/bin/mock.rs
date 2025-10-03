
use back_judge_grpc::{
    generated::judge_service_server::JudgeServiceServer, server::WrappedJudgeService,
};
use judge_core::logic::judge_service_impl::JudgeServiceImpl;
use judge_infra_mock::job_service::job_service::JobService;
use judge_infra_mock::multi_proc_problem_registry::registry_client::RegistryClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let problem_registry_dir = std::env::var("TRAOJUDGE_PROBLEM_REGISTRY_DIR")
        .expect("TRAOJUDGE_PROBLEM_REGISTRY_DIR must be set");
    let job_service_cache_dir = std::env::var("TRAOJUDGE_JOB_SERVICE_CACHE_DIR")
        .expect("TRAOJUDGE_JOB_SERVICE_CACHE_DIR must be set");
    let container_dir = std::env::var("TRAOJUDGE_CONTAINER_DIR")
        .expect("TRAOJUDGE_CONTAINER_DIR must be set");
    let grpc_service_port = std::env::var("TRAOJUDGE_GRPC_SERVICE_PORT")
        .expect("TRAOJUDGE_GRPC_SERVICE_PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse TRAOJUDGE_GRPC_SERVICE_PORT");
    let grpc_service_addr = format!("127.0.0.1:{}", grpc_service_port)
        .parse::<std::net::SocketAddr>()
        .expect("Failed to parse grpc service address");
    tracing::info!("problem_registry_dir: {}", problem_registry_dir);
    tracing::info!("job_service_cache_dir: {}", job_service_cache_dir);
    tracing::info!("grpc_service_addr: {}", grpc_service_addr);
    let problem_registry_client = RegistryClient::new(problem_registry_dir.into());
    tracing::info!("ProblemRegistryClient created");
    let job_service = JobService::new(job_service_cache_dir.into(), container_dir.into(), problem_registry_client, "trao-mock-exec:latest".to_string())
        .expect("Failed to create JobService");
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
