#![allow(unused)]
use judge_core::model::*;

#[derive(Clone)]
pub struct ProblemRegistryServer {}

#[axum::async_trait]
impl problem_registry::ProblemRegistryServer for ProblemRegistryServer {
    async fn register_many(
        &self,
        resource_id_to_content: std::collections::HashMap<identifiers::ResourceId, String>,
    ) -> Result<(), problem_registry::RegistrationError> {
        todo!()
    }

    async fn restore_name(&self, dep_id: identifiers::DepId) -> Option<String> {
        todo!()
    }

    async fn remove(
        &self,
        procedure: procedure::registered::Procedure,
    ) -> Result<(), problem_registry::RemovalError> {
        todo!()
    }
}
