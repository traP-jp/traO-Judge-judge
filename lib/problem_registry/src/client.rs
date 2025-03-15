#![allow(unused)]
use judge_core::model::problem_registry;

#[derive(Clone)]
pub struct ProblemRegistryClient {}

#[axum::async_trait]
impl problem_registry::ProblemRegistryClient for ProblemRegistryClient {
    async fn fetch(
        &self,
        resource_id: judge_core::model::identifiers::ResourceId,
    ) -> Result<String, problem_registry::ResourceFetchError> {
        todo!()
    }
}
