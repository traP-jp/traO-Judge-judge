use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::Client as S3Client;
use judge_core::model::problem_registry;
use std::env;

#[derive(Clone)]
pub struct ProblemRegistryClient {
    s3_client: S3Client,
}

impl ProblemRegistryClient {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let config = aws_config::from_env().region(region_provider).load().await;
        Self {
            s3_client: S3Client::new(&config),
        }
    }
}

#[axum::async_trait]
impl problem_registry::ProblemRegistryClient for ProblemRegistryClient {
    async fn fetch(
        &self,
        resource_id: judge_core::model::identifiers::ResourceId,
    ) -> Result<String, problem_registry::ResourceFetchError> {
        // checked in lib/jobapi/src/aws.rs
        let judge_bucket_name = env::var("JUDGE_BUCKET_NAME").unwrap();
        let s3_response = self
            .s3_client
            .get_object()
            .bucket(judge_bucket_name)
            .key(resource_id.to_string())
            .send()
            .await;
        let mut s3_response = match s3_response {
            Ok(_) => s3_response.unwrap(),
            Err(SdkError::ServiceError(err)) => {
                if err.err().is_no_such_key() {
                    return Err(problem_registry::ResourceFetchError::NotFound(resource_id));
                }
                return Err(problem_registry::ResourceFetchError::FetchFailed(
                    err.err().to_string(),
                ));
            }
            Err(err) => {
                return Err(problem_registry::ResourceFetchError::FetchFailed(
                    err.to_string(),
                ));
            }
        };
        let bytes: Vec<u8> = match s3_response.body.next().await {
            None => {
                return Err(problem_registry::ResourceFetchError::FetchFailed(
                    "Content is empty".to_string(),
                ));
            }
            Some(Ok(bytes)) => bytes.to_vec(),
            Some(Err(err)) => {
                return Err(problem_registry::ResourceFetchError::FetchFailed(
                    err.to_string(),
                ));
            }
        };
        let content = String::from_utf8(bytes);
        match content {
            Ok(content) => Ok(content),
            Err(_) => Err(problem_registry::ResourceFetchError::FetchFailed(
                "Failed to parse content".to_string(),
            )),
        }
    }
}
