use crate::procedure_builder::PyProcedureBuilder;
use judge_core::{
    logic::*,
    model::{problem_registry::ProblemRegistryServer as _, *},
};
use local_jobapi::jobapi::JobApi;
use local_problem_registry::{
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
    jobapi: JobApi<RegistryClient>,
    registry_server: RegistryServer,
}

#[gen_stub_pymethods]
#[pymethods]
impl LocalJudge {
    #[new]
    fn new(temp_dir: PathBuf) -> Self {
        let (server, client) = new_registry();
        let jobapi = JobApi::new(temp_dir, client).unwrap();
        LocalJudge {
            jobapi,
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
        let (runtime_procedure, runtime_id_to_dep_id) =
            registered_procedure_converter::convert(&registered_procedure, &runtime_text_contents)
                .unwrap();
        let runner = runner::Runner::new(self.jobapi.clone(), runtime_procedure)
            .await
            .unwrap();
        let result = runner.run().await.unwrap();
        let mut result_in_dep_id = HashMap::new();
        for (runtime_id, outcome) in result {
            let dep_id = runtime_id_to_dep_id.get(&runtime_id).unwrap();
            result_in_dep_id.insert(dep_id.clone(), outcome);
        }
        let mut result_in_name = HashMap::new();
        for (dep_id, outcome) in result_in_dep_id {
            let name = self.registry_server.restore_name(dep_id).await.unwrap();
            result_in_name.insert(name, outcome);
        }
        let result_stringified: String = serde_json::to_string(&result_in_name).unwrap();
        Ok(result_stringified)
    }
}
