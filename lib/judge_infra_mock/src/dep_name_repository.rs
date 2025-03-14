use judge_core::model::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DepNameRepository {
    dep_names: Arc<Mutex<HashMap<identifiers::DepId, String>>>,
}

impl DepNameRepository {
    pub fn new() -> Self {
        Self {
            dep_names: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[axum::async_trait]
impl dep_name_repository::DepNameRepository for DepNameRepository {
    async fn insert_many(&self, dep_id_to_name: HashMap<identifiers::DepId, String>) -> Result<()> {
        {
            let mut dep_names = self.dep_names.lock().await;
            for (dep_id, name) in dep_id_to_name {
                dep_names.insert(dep_id, name);
            }
            std::mem::drop(dep_names);
        }
        Ok(())
    }

    async fn get_many(&self, dep_ids: Vec<identifiers::DepId>) -> HashMap<identifiers::DepId, Option<String>> {
        let dep_id_to_name = {
            let mut dep_id_to_name = HashMap::new();
            let dep_names = self.dep_names.lock().await;
            for dep_id in dep_ids {
                dep_id_to_name.insert(dep_id, dep_names.get(&dep_id).cloned());
            }
            std::mem::drop(dep_names);
            dep_id_to_name
        };
        dep_id_to_name
    }

    async fn remove_many(&self, dep_ids: Vec<identifiers::DepId>) -> Result<()> {
        {
            let mut dep_names = self.dep_names.lock().await;
            for dep_id in dep_ids {
                dep_names.remove(&dep_id);
            }
            std::mem::drop(dep_names);
        }
        Ok(())
    }
}