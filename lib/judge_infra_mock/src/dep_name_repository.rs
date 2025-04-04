use anyhow::Result;
use judge_core::model::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct DepNameRepository {
    dep_names: Arc<Mutex<HashMap<identifiers::DepId, String>>>,
    problem_id_to_dep_ids: Arc<Mutex<HashMap<i64, Vec<identifiers::DepId>>>>,
}

impl DepNameRepository {
    pub fn new() -> Self {
        Self {
            dep_names: Arc::new(Mutex::new(HashMap::new())),
            problem_id_to_dep_ids: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[axum::async_trait]
impl dep_name_repository::DepNameRepository<i64> for DepNameRepository {
    async fn insert_many(
        &self,
        _problem_id: i64,
        dep_id_to_name: HashMap<identifiers::DepId, String>,
    ) -> Result<()> {
        let dep_ids = {
            let mut dep_ids = Vec::new();
            let mut dep_names = self.dep_names.lock().await;
            for (dep_id, name) in dep_id_to_name {
                dep_names.insert(dep_id, name);
                dep_ids.push(dep_id);
            }
            std::mem::drop(dep_names);
            dep_ids
        };
        {
            let mut problem_id_to_dep_ids = self.problem_id_to_dep_ids.lock().await;
            problem_id_to_dep_ids.insert(_problem_id, dep_ids);
            std::mem::drop(problem_id_to_dep_ids);
        }
        Ok(())
    }

    async fn get_many(
        &self,
        dep_ids: Vec<identifiers::DepId>,
    ) -> HashMap<identifiers::DepId, Option<String>> {
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

    async fn remove_many(&self, problem_id: i64) -> Result<()> {
        let dep_ids = {
            let mut problem_id_to_dep_ids = self.problem_id_to_dep_ids.lock().await;
            let dep_ids = if let Some(dep_ids) = problem_id_to_dep_ids.get(&problem_id) {
                dep_ids.clone()
            } else {
                return Err(anyhow::anyhow!("Problem ID {} not found", problem_id));
            };
            problem_id_to_dep_ids.remove(&problem_id);
            std::mem::drop(problem_id_to_dep_ids);
            dep_ids
        };
        {
            let mut dep_names = self.dep_names.lock().await;
            for dep_id in dep_ids {
                dep_names.remove(&dep_id);
            }
            std::mem::drop(dep_names);
        }
        Ok(())
    }

    async fn get_many_by_problem_id(
        &self,
        problem_id: i64,
    ) -> Result<HashMap<identifiers::DepId, String>> {
        let dep_ids = {
            let problem_id_to_dep_ids = self.problem_id_to_dep_ids.lock().await;
            if let Some(dep_ids) = problem_id_to_dep_ids.get(&problem_id) {
                dep_ids.clone()
            } else {
                return Err(anyhow::anyhow!("Problem ID {} not found", problem_id));
            }
        };
        let dep_to_name = {
            let dep_names = self.dep_names.lock().await;
            let mut dep_to_name = HashMap::new();
            for dep_id in dep_ids {
                if let Some(name) = dep_names.get(&dep_id) {
                    dep_to_name.insert(dep_id, name.clone());
                } else {
                    return Err(anyhow::anyhow!("Dependency ID {} not found", dep_id));
                }
            }
            std::mem::drop(dep_names);
            dep_to_name
        };
        Ok(dep_to_name)
    }
}
