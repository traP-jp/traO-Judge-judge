#![allow(unused)]
use judge_core::model::*;

#[derive(Clone)]
pub struct ProblemRegistryServer {}

#[axum::async_trait]
impl problem_registry::ProblemRegistryServer for ProblemRegistryServer {
    async fn register(
        &self,
        resource_id: identifiers::ResourceId,
        content: String,
    ) -> Result<(), problem_registry::RegistrationError> {
        todo!()
    }

    async fn remove(
        &self,
        resource_id: identifiers::ResourceId,
    ) -> Result<(), problem_registry::RemovalError> {
        todo!()
    }
}
