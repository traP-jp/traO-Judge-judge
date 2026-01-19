use judge_core::model::{problem_registry::*, procedure::*, *};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RegistryServer {
    pub(crate) registry: Arc<Mutex<HashMap<identifiers::ResourceId, String>>>,
}

#[axum::async_trait]
impl ProblemRegistryServer for RegistryServer {
    async fn register(
        &self,
        resource_id: identifiers::ResourceId,
        content: String,
    ) -> Result<(), RegistrationError> {
        {
            let mut registry = self.registry.lock().await;
            registry.insert(resource_id, content);
            std::mem::drop(registry);
        }
        Ok(())
    }

    async fn remove(&self, resource_id: identifiers::ResourceId) -> Result<(), RemovalError> {
        let mut registry = self.registry.lock().await;
        registry
            .remove(&resource_id)
            .ok_or(RemovalError::NotFound(resource_id))?;
        Ok(())
    }
}
