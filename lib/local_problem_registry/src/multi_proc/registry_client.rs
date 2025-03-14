use judge_core::model::*;

#[derive(Debug, Clone)]
pub struct RegistryClient {
    cache_dir: std::path::PathBuf,
}

impl RegistryClient {
    pub fn new(cache_dir: std::path::PathBuf) -> Self {
        Self { cache_dir }
    }
}

#[axum::async_trait]
impl problem_registry::ProblemRegistryClient for RegistryClient {
    async fn fetch(
        &self,
        resource_id: identifiers::ResourceId,
    ) -> Result<String, problem_registry::ResourceFetchError> {
        let uuid: uuid::Uuid = resource_id.into();
        let path = self.cache_dir.join(uuid.to_string());
        match tokio::fs::read_to_string(path).await {
            Ok(contents) => Ok(contents),
            Err(_) => Err(problem_registry::ResourceFetchError::NotFound(resource_id)),
        }
    }
}
