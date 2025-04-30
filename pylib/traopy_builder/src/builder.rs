use judge_core::logic::{
    self, judge_service_impl::JudgeServiceImpl, procedure_builder::ProcedureBuilder,
};
use judge_core::model::{
    dep_name_repository::DepNameRepository as _,
    judge::{JudgeRequest, JudgeService as _},
    procedure::writer_schema::{self, *},
};
use judge_infra_mock::{
    dep_name_repository::DepNameRepository, jobapi::jobapi::JobApi,
    one_proc_problem_registry::new_registry,
};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[pyclass(module = "traopy_builder.builder")]
#[gen_stub_pyclass]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Outcome {
    pub(crate) id: Uuid,
}

#[pyclass(module = "traopy_builder.builder")]
#[gen_stub_pyclass]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Dependency {
    pub(crate) id: Uuid,
    pub(crate) envvar_name: String,
}

#[pymethods]
#[gen_stub_pymethods]
impl Dependency {
    #[new]
    fn new(outcome: &Outcome, envvar_name: String) -> Self {
        Dependency {
            id: outcome.id,
            envvar_name,
        }
    }
}

#[pyclass(module = "traopy_builder.builder")]
#[gen_stub_pyclass]
pub struct Builder {
    inner: ProcedureBuilder,
    id_to_name: HashMap<Uuid, String>,
}

#[pymethods]
#[gen_stub_pymethods]
impl Builder {
    #[new]
    fn new() -> Self {
        Builder {
            inner: ProcedureBuilder::new(),
            id_to_name: HashMap::new(),
        }
    }

    fn add_static_text(&mut self, name: String, content: String) -> PyResult<Outcome> {
        let text = Text {
            name: name.clone(),
            content,
        };
        let resource = ResourceKind::TextFile(text);
        let id = uuid::Uuid::new_v4();
        self.inner.add_resource(resource).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Resource {} already exists",
                name
            ))
        })?;
        self.id_to_name.insert(id, name);
        Ok(Outcome { id })
    }

    fn add_runtime_text(&mut self, name: String, label: String) -> PyResult<Outcome> {
        let runtime_text = RuntimeText {
            name: name.clone(),
            label,
        };
        let resource = ResourceKind::RuntimeTextFile(runtime_text);
        let id = uuid::Uuid::new_v4();
        self.inner.add_resource(resource).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Resource {} already exists",
                name
            ))
        })?;
        self.id_to_name.insert(id, name);
        Ok(Outcome { id })
    }

    fn add_empty_directory(&mut self, name: String) -> PyResult<Outcome> {
        let empty_directory = EmptyDirectory { name: name.clone() };
        let resource = ResourceKind::EmptyDirectory(empty_directory);
        let id = uuid::Uuid::new_v4();
        self.inner.add_resource(resource).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Resource {} already exists",
                name
            ))
        })?;
        self.id_to_name.insert(id, name);
        Ok(Outcome { id })
    }

    fn add_script(&mut self, name: String, content: String) -> PyResult<Outcome> {
        let script = Text {
            name: name.clone(),
            content,
        };
        let id = uuid::Uuid::new_v4();
        self.inner.add_script(script).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Script {} already exists",
                name
            ))
        })?;
        self.id_to_name.insert(id, name);
        Ok(Outcome { id })
    }

    fn add_execution(
        &mut self,
        name: String,
        script: Outcome,
        dependencies: Vec<Dependency>,
        time_reserved_ms: u64,
    ) -> PyResult<Outcome> {
        let mut inner_dependencies = Vec::new();
        for dep in dependencies {
            let dep_name = self.id_to_name.get(&dep.id).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Dependency {} not found",
                    dep.id
                ))
            })?;
            let dependency = writer_schema::Dependency {
                ref_to: dep_name.clone(),
                envvar_name: dep.envvar_name,
            };
            inner_dependencies.push(dependency);
        }
        let script_name = self
            .id_to_name
            .get(&script.id)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Script {} not found",
                    script.id
                ))
            })?
            .clone();
        let execution = Execution {
            name: name.clone(),
            script_name: script_name.clone(),
            dependencies: inner_dependencies,
            time_reserved_ms,
        };
        let id = uuid::Uuid::new_v4();
        self.inner.add_execution(execution).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Execution {} already exists",
                name
            ))
        })?;
        self.id_to_name.insert(id, name);
        Ok(Outcome { id })
    }

    fn jsonify(&self) -> PyResult<String> {
        let procedure = self.inner.get_procedure();
        let json = serde_json::to_string(&procedure).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to serialize procedure")
        })?;
        Ok(json)
    }

    fn run(
        &self,
        label_to_content: HashMap<String, String>,
        host_temp_dir: PathBuf,
        container_temp_dir: PathBuf,
        container_image_name: String,
    ) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to create runtime: {:?}",
                e
            ))
        })?;
        rt.block_on(async {
            self.run_internal(
                label_to_content,
                host_temp_dir,
                container_temp_dir,
                container_image_name,
            )
            .await
        })
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to run procedure: {:?}",
                e
            ))
        })
    }
}

impl Builder {
    async fn run_internal(
        &self,
        label_to_content: HashMap<String, String>,
        host_temp_dir: PathBuf,
        container_temp_dir: PathBuf,
        container_image_name: String,
    ) -> PyResult<String> {
        let writer_procedure = self.inner.get_procedure();
        let (regi_server, regi_client) = new_registry();
        let dn_repo = DepNameRepository::new();
        let job_api = JobApi::new(
            host_temp_dir,
            container_temp_dir,
            regi_client,
            container_image_name.clone(),
        )
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to create JobApi"))?;
        let regi_procedure = logic::writer_schema_registerer::register(
            writer_procedure,
            regi_server,
            dn_repo.clone(),
            0 as i64,
        )
        .await
        .map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to register procedure")
        })?;
        let judge_service = JudgeServiceImpl::new(job_api);
        let judge_req = JudgeRequest {
            procedure: regi_procedure,
            runtime_texts: label_to_content,
        };
        let judge_resp = judge_service.judge(judge_req).await.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to judge: {:?}", e))
        })?;
        let judge_resp_with_name = {
            let dep_to_name = dn_repo
                .get_many(judge_resp.keys().cloned().collect())
                .await
                .into_iter()
                .map(|(id, name)| match name {
                    Some(name) => Ok((id, name)),
                    None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Dependency {} not found",
                        id
                    ))),
                })
                .collect::<Result<HashMap<_, _>, _>>()?;
            judge_resp
                .into_iter()
                .map(|(id, result)| match dep_to_name.get(&id) {
                    Some(name) => Ok((name.clone(), result)),
                    None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Dependency {} not found",
                        id
                    ))),
                })
                .collect::<Result<HashMap<_, _>, _>>()?
        };
        let json = serde_json::to_string(&judge_resp_with_name).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to serialize judge response")
        })?;
        Ok(json)
    }
}
