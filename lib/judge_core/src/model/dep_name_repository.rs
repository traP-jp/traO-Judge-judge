use super::identifiers;
use anyhow::Result;
use std::collections::HashMap;

#[axum::async_trait]
pub trait DepNameRepository: Clone + Send + Sync {
    async fn insert_many(&self, dep_id_to_name: HashMap<identifiers::DepId, String>) -> Result<()>;
    async fn get_many(
        &self,
        dep_ids: Vec<identifiers::DepId>,
    ) -> HashMap<identifiers::DepId, Option<String>>;
    async fn remove_many(&self, dep_ids: Vec<identifiers::DepId>) -> Result<()>;
}
