use super::identifiers::ResourceId;

/// ProblemRegistryServer uploads contents of problems to the registry in webservice-backend server.
#[axum::async_trait]
pub trait ProblemRegistryServer: Clone + Send + Sync {
    // Memo: use crate::writer_schema_transpiler::transpile as the core logic
    async fn register(
        &self,
        resource_id: ResourceId,
        content: String,
    ) -> Result<(), RegistrationError>;

    async fn remove(&self, resource_id: ResourceId) -> Result<(), RemovalError>;
}

/// ProblemRegistryClient fetches contents of problems from the registry in judge server.
#[axum::async_trait]
pub trait ProblemRegistryClient: Clone + Send + Sync + 'static {
    async fn fetch(&self, resource_id: ResourceId) -> Result<String, ResourceFetchError>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ResourceFetchError {
    #[error("Failed to fetch resource with error: {0}")]
    FetchFailed(String),
    #[error("Resource {0} not found")]
    NotFound(ResourceId),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum RegistrationError {
    #[error("Internal error while registering a problem: {0}")]
    InternalError(String),
    #[error("Invalid problem procedure schema: {0}")]
    InvalidSchema(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum RemovalError {
    #[error("Internal error while removing a problem: {0}")]
    InternalError(String),
    #[error("Resource {0} not found")]
    NotFound(ResourceId),
}
