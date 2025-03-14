use crate::procedure_builder::PyProcedureBuilder;
use judge_core::{
    logic::*,
    model::{
        judge::JudgeApi as _,
        problem_registry::{ProblemRegistryClient, ProblemRegistryServer as _},
        *,
    },
};
use local_jobapi::{
    jobapi::JobApi,
    tokens::{OutcomeToken, RegistrationToken},
};
use local_problem_registry::one_proc::{
    new_registry, registry_client::RegistryClient, registry_server::RegistryServer,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// LocalJudge object to run provided procedures in your local environment.
#[gen_stub_pyclass]
#[pyclass]
pub struct LocalJudge {
    registry_server: RegistryServer,
    judge_api:
        judge_api_impl::JudgeApiImpl<RegistrationToken, OutcomeToken, JobApi<RegistryClient>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl LocalJudge {
    #[new]
    fn new(temp_dir: PathBuf) -> Self {
        let (server, client) = new_registry();
        let jobapi = JobApi::new(temp_dir, client).unwrap();
        let judge_api = judge_api_impl::JudgeApiImpl::new(jobapi);
        LocalJudge {
            judge_api,
            registry_server: server,
        }
    }

    /// Run the provided procedure in your local environment.
    #[pyo3(name = "run")]
    async fn run(
        &self,
        builder: PyProcedureBuilder,
        runtime_text_contents: HashMap<String, String>,
    ) -> PyResult<String> {
        let procedure: procedure::writer_schema::Procedure = builder.get_schema_procedure();
        let registered_procedure = self.registry_server.register(procedure).await.unwrap();
        let judge_request = judge::JudgeRequest {
            procedure: registered_procedure,
            runtime_texts: runtime_text_contents,
        };
        let result = self.judge_api.judge(judge_request).await.unwrap();
        let mut result_in_name = HashMap::new();
        for (dep_id, outcome) in result {
            let name = self.registry_server.restore_name(dep_id).await.unwrap();
            result_in_name.insert(name, outcome);
        }
        let result_stringified: String = serde_json::to_string(&result_in_name).unwrap();
        Ok(result_stringified)
    }
}
