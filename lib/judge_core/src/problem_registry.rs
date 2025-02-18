use crate::{identifiers::ResourceId, procedure::*};
use futures::Future;

/// ProblemRegistryServer uploads contents of problems to the registry in webservice-backend server.
pub trait ProblemRegistryServer {
    // Memo: use crate::writer_schema_transpiler::transpile as the core logic
    fn register(
        &self,
        problem: writer_schema::Procedure,
    ) -> impl Future<Output = Result<registered::Procedure, RegistrationError>>;
}

/// ProblemRegistryClient fetches contents of problems from the registry in judge server.
pub trait ProblemRegistryClient {
    fn fetch(
        &self,
        resource_id: ResourceId,
    ) -> impl Future<Output = Result<String, ResourceFetchError>>;
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
