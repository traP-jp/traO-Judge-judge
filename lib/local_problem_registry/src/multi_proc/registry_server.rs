use judge_core::{
    logic::writer_schema_transpiler::transpile,
    model::{problem_registry::*, procedure::*, *},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RegistryServer {
    cache_dir: std::path::PathBuf,
    dep_id_to_name: Arc<Mutex<HashMap<identifiers::DepId, String>>>,
}

#[axum::async_trait]
impl ProblemRegistryServer for RegistryServer {
    async fn register(
        &self,
        procedure: writer_schema::Procedure,
    ) -> Result<registered::Procedure, RegistrationError> {
        let (transpiled_procedure, content_to_register, dep_id_to_name) = transpile(procedure)?;
        for (resource_id, content) in content_to_register {
            let uuid: uuid::Uuid = resource_id.into();
            let path = self.cache_dir.join(uuid.to_string());
            if tokio::fs::try_exists(&path).await.is_ok() {
                return Err(RegistrationError::InternalError(format!(
                    "Resource {} already exists",
                    resource_id
                )));
            }
            tokio::fs::write(path, content).await.map_err(|e| {
                RegistrationError::InternalError(format!("Failed to write to cache: {}", e))
            })?;
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
        for text in procedure.texts {
            let uuid: uuid::Uuid = text.resource_id.into();
            let path = self.cache_dir.join(uuid.to_string());
            tokio::fs::remove_file(path).await.map_err(|e| {
                RemovalError::InternalError(format!("Failed to remove from cache: {}", e))
            })?;
        }
        Ok(())
    }
}
