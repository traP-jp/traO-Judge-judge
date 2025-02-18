use judge_core::{problem_registry::*, procedure::*, writer_schema_transpiler::transpile, *};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RegistryServer {
    pub(crate) registry: Arc<Mutex<HashMap<identifiers::ResourceId, String>>>,
}

impl ProblemRegistryServer for RegistryServer {
    async fn register(
        &self,
        procedure: writer_schema::Procedure,
    ) -> Result<registered::Procedure, RegistrationError> {
        let (transpiled_procedure, content_to_register) = transpile(procedure)?;
        let mut registry = self.registry.lock().await;
        for (resource_id, content) in content_to_register {
            registry.insert(resource_id, content);
        }
        Ok(transpiled_procedure)
    }
}
