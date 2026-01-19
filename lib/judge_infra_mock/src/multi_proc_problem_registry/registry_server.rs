use judge_core::model::{problem_registry::*, procedure::*, *};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RegistryServer {
    cache_dir: std::path::PathBuf,
    dep_id_to_name: Arc<Mutex<HashMap<identifiers::DepId, String>>>,
}

impl RegistryServer {
    pub fn new(cache_dir: std::path::PathBuf) -> Self {
        Self {
            cache_dir,
            dep_id_to_name: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[axum::async_trait]
impl ProblemRegistryServer for RegistryServer {
    async fn register(
        &self,
        resource_id: identifiers::ResourceId,
        content: String,
    ) -> Result<(), RegistrationError> {
        let uuid: uuid::Uuid = resource_id.into();
        let path = self.cache_dir.join(uuid.to_string());
        if tokio::fs::try_exists(&path).await.unwrap_or(true) {
            return Err(RegistrationError::InternalError(format!(
                "Resource {} already exists",
                resource_id
            )));
        }
        tokio::fs::write(path, content).await.map_err(|e| {
            RegistrationError::InternalError(format!("Failed to write to cache: {}", e))
        })?;

        Ok(())
    }

    async fn remove(&self, resource_id: identifiers::ResourceId) -> Result<(), RemovalError> {
        let uuid: uuid::Uuid = resource_id.into();
        let path = self.cache_dir.join(uuid.to_string());
        tokio::fs::remove_file(path).await.map_err(|e| {
            RemovalError::InternalError(format!("Failed to remove from cache: {}", e))
        })?;

        Ok(())
    }
}
