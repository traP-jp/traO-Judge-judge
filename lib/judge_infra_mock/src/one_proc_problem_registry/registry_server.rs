use judge_core::model::{problem_registry::*, procedure::*, *};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RegistryServer {
    pub(crate) registry: Arc<Mutex<HashMap<identifiers::ResourceId, String>>>,
    pub(crate) dep_id_to_name: Arc<Mutex<HashMap<identifiers::DepId, String>>>,
}

#[axum::async_trait]
impl ProblemRegistryServer for RegistryServer {
    async fn register_many(
        &self,
        resource_id_to_content: HashMap<identifiers::ResourceId, String>,
    ) -> Result<(), RegistrationError> {
        {
            let mut registry = self.registry.lock().await;
            for (resource_id, content) in resource_id_to_content {
                registry.insert(resource_id, content);
            }
            std::mem::drop(registry);
        }
        Ok(())
    }

    async fn restore_name(&self, dep_id: identifiers::DepId) -> Option<String> {
        let dep_id_to_name = self.dep_id_to_name.lock().await;
        dep_id_to_name.get(&dep_id).cloned()
    }

    async fn remove(&self, procedure: registered::Procedure) -> Result<(), RemovalError> {
        let mut registry = self.registry.lock().await;
        for text in procedure.texts {
            registry.remove(&text.resource_id);
        }
        Ok(())
    }
}
