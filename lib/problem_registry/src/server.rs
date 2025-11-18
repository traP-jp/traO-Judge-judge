#![allow(unused)]
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client as S3Client, error::SdkError, primitives::ByteStream};
use judge_core::model::*;

#[derive(Clone)]
pub struct ProblemRegistryServer {
    s3_client: S3Client,
}

impl ProblemRegistryServer {
    pub async fn new() -> Self {
        // FIXME: do not hard code!
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        Self {
            s3_client: S3Client::new(&config),
        }
    }
}

#[axum::async_trait]
impl problem_registry::ProblemRegistryServer for ProblemRegistryServer {
    async fn register(
        &self,
        resource_id: identifiers::ResourceId,
        content: String,
    ) -> Result<(), problem_registry::RegistrationError> {
        // FIXME: do not hard code!
        let judge_bucket_name = std::env::var("JUDGE_BUCKET_NAME").unwrap();

        // TODO: check if content is valid

        let s3_response = self
            .s3_client
            .put_object()
            .bucket(judge_bucket_name)
            .key(resource_id.to_string())
            .body(ByteStream::from(content.into_bytes()))
            .send()
            .await
            .map_err(|e| {
                problem_registry::RegistrationError::InternalError(format!(
                    "registration failed: {e}"
                ))
            })?;
        // TODO: logging for s3_response
        Ok(())
    }

    async fn remove(
        &self,
        resource_id: identifiers::ResourceId,
    ) -> Result<(), problem_registry::RemovalError> {
        // FIXME: do not hard code!
        let judge_bucket_name = std::env::var("JUDGE_BUCKET_NAME").unwrap();

        // TODO: check if found or not (DeleteObjectError doesn't have NoSuchKey)

        let s3_response = self
            .s3_client
            .delete_object()
            .bucket(judge_bucket_name)
            .key(resource_id.to_string())
            .send()
            .await
            .map_err(|e| {
                problem_registry::RemovalError::InternalError(format!("removal failed: {e}"))
            })?;
        // TODO: logging for s3_response
        Ok(())
    }
}
