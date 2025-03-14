use local_problem_registry::multi_proc::registry_client::RegistryClient;
use local_jobapi::{
    jobapi::JobApi,
    tokens::{
        OutcomeToken,
        RegistrationToken,
    }
};
use judge_core::logic::judge_api_impl::JudgeApiImpl;
use back_judge_grpc::{
    generated::judge_service_server::JudgeServiceServer,
    server::WrappedJudgeApi,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let problem_registry_dir = std::env::var("TRAOJUDGE_PROBLEM_REGISTRY_DIR")
        .expect("TRAOJUDGE_PROBLEM_REGISTRY_DIR must be set");
    let jobapi_cache_dir = std::env::var("TRAOJUDGE_JOBAPI_CACHE_DIR")
        .expect("TRAOJUDGE_JOBAPI_CACHE_DIR must be set");
    let grpc_service_port = std::env::var("TRAOJUDGE_GRPC_SERVICE_PORT")
        .expect("TRAOJUDGE_GRPC_SERVICE_PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse TRAOJUDGE_GRPC_SERVICE_PORT");
    let grpc_service_addr = format!("127.0.0.1:{}", grpc_service_port)
        .parse::<std::net::SocketAddr>()
        .expect("Failed to parse grpc service address");
    tracing::info!("problem_registry_dir: {}", problem_registry_dir);
    tracing::info!("jobapi_cache_dir: {}", jobapi_cache_dir);
    tracing::info!("grpc_service_addr: {}", grpc_service_addr);
    let problem_registry_client = RegistryClient::new(problem_registry_dir.into());
    tracing::info!("ProblemRegistryClient created");
    let jobapi = JobApi::new(
        jobapi_cache_dir.into(),
        problem_registry_client
    )
        .expect("Failed to create JobApi");
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
