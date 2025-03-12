use judge_core::{
    model::{problem_registry::*, procedure::*, *},
    logic::writer_schema_transpiler::transpile
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RegistryServer {
    pub(crate) registry: Arc<Mutex<HashMap<identifiers::ResourceId, String>>>,
    pub(crate) dep_id_to_name: Arc<Mutex<HashMap<identifiers::DepId, String>>>,
}

#[axum::async_trait]
impl ProblemRegistryServer for RegistryServer {
    async fn register(
        &self,
        procedure: writer_schema::Procedure,
    ) -> Result<registered::Procedure, RegistrationError> {
        let (transpiled_procedure, content_to_register, dep_id_to_name) = transpile(procedure)?;
        {
            let mut registry = self.registry.lock().await;
            for (resource_id, content) in content_to_register {
                registry.insert(resource_id, content);
            }
        }
        {
            let mut dep_id_to_name_global = self.dep_id_to_name.lock().await;
            for (dep_id, name) in dep_id_to_name {
                dep_id_to_name_global.insert(dep_id, name);
            }
        }
        Ok(transpiled_procedure)
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
