use crate::{identifiers::ResourceId, procedure::*};
use anyhow::Result;
use futures::Future;

/// ProblemRegistryServer uploads contents of problems to the registry in webservice-backend server.
pub trait ProblemRegistryServer {
    fn register(
        &self,
        problem: writer_schema::Procedure,
    ) -> impl Future<Output = Result<registered::Procedure>>;
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
