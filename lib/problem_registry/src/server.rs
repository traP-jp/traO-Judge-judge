#![allow(unused)]
use judge_core::*;

#[derive(Clone)]
pub struct ProblemRegistryServer {}

impl problem_registry::ProblemRegistryServer for ProblemRegistryServer {
    async fn register(
        &self,
        problem: procedure::writer_schema::Procedure,
    ) -> Result<procedure::registered::Procedure, problem_registry::RegistrationError> {
        todo!()
    }

    async fn restore_name(&self, dep_id: identifiers::DepId) -> Option<String> {
        todo!()
    }
}
