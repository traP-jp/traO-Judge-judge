#![allow(unused)]
use judge_core::problem_registry;

#[derive(Clone)]
pub struct ProblemRegistryClient {}

impl problem_registry::ProblemRegistryClient for ProblemRegistryClient {
    async fn fetch(
        &self,
        resource_id: judge_core::identifiers::ResourceId,
    ) -> Result<String, problem_registry::ResourceFetchError> {
        todo!()
    }
}
