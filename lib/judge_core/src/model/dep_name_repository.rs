use super::identifiers;
use anyhow::Result;
use std::collections::HashMap;

#[axum::async_trait]
pub trait DepNameRepository<IdType>: Clone + Send + Sync {
    async fn insert_many(
        &self,
        problem_id: IdType,
        dep_id_to_name: HashMap<identifiers::DepId, DepNameRecord>,
    ) -> Result<()>;
    async fn get_many(
        &self,
        dep_ids: Vec<identifiers::DepId>,
    ) -> Result<HashMap<identifiers::DepId, Option<DepNameRecord>>>;
    async fn remove_many(&self, problem_id: IdType) -> Result<()>;
    async fn get_many_by_problem_id(
        &self,
        problem_id: IdType,
    ) -> Result<HashMap<identifiers::DepId, DepNameRecord>>;
}

pub struct DepNameRecord {
    pub name: String,
    pub hash_sha256: String,
}
