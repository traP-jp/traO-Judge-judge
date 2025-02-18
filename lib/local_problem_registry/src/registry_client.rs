use judge_core::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RegistryClient {
    pub(crate) registry: Arc<Mutex<HashMap<identifiers::ResourceId, String>>>,
}

impl problem_registry::ProblemRegistryClient for RegistryClient {
    async fn fetch(
        &self,
        resource_id: identifiers::ResourceId,
    ) -> Result<String, problem_registry::ResourceFetchError> {
        let registry = self.registry.lock().await;
        match registry.get(&resource_id) {
            Some(contents) => Ok(contents.clone()),
            None => Err(problem_registry::ResourceFetchError::NotFound(resource_id)),
        }
    }
}
