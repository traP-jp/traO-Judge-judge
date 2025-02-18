use crate::models::*;
use core::panic;
use judge_core::procedure::writer_schema;
use judge_core::procedure::writer_schema::*;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyProcedure {
    resources: HashMap<String, ResourceKind>,
    executions: HashMap<String, Execution>,
    scripts: HashMap<String, Script>,
}

impl From<PyProcedure> for Procedure {
    fn from(py_procedure: PyProcedure) -> Self {
        Self {
            resources: py_procedure.resources.clone(),
            executions: py_procedure.executions.clone(),
            scripts: py_procedure.scripts.clone(),
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyProcedure {
    #[new]
    fn new() -> Self {
        PyProcedure {
            resources: HashMap::new(),
            executions: HashMap::new(),
            scripts: HashMap::new(),
        }
    }

    #[pyo3(name = "add_resource")]
    fn add_resource(&mut self, resource: resource_kind::PyResourceKind) -> output::PyOutput {
        let schema_resource = writer_schema::ResourceKind::from(resource);
        let name = resource_kind::resource_name(&schema_resource);
        let _ = self
            .resources
            .insert(name.clone(), schema_resource)
            .is_none_or(|_| {
                panic!("Resource with name {} already exists", name);
            });
        let output = output::PyOutput::new(name);
        output
    }

    #[pyo3(name = "add_script")]
    fn add_script(&mut self, script: script::PyScript) -> output::PyScriptOutput {
        let schema_script = writer_schema::Script::from(script);
        let name = schema_script.name.clone();
        let _ = self
            .scripts
            .insert(name.clone(), schema_script)
            .is_none_or(|_| {
                panic!("Script with name {} already exists", name);
            });
        let output = output::PyScriptOutput::new(name);
        output
    }

    #[pyo3(name = "add_execution")]
    fn add_execution(&mut self, execution: execution::PyExecution) -> output::PyOutput {
        let _ = self.scripts.get(&execution.script.name).unwrap();
        let script_name = execution.script.name.clone();
        let dependencies = execution
            .depends_on
            .iter()
            .map(|dep| {
                let _ = self.resources.get(&dep.ref_to.name).unwrap().clone();
                let schema_dep = writer_schema::Dependency::from(dep.clone());
                schema_dep
            })
            .collect::<Vec<_>>();
        let schema_execution = execution::new_execution(execution.name, script_name, dependencies);
        let name = schema_execution.name.clone();
        let _ = self
            .executions
            .insert(name.clone(), schema_execution)
            .is_none_or(|_| {
                panic!("Execution with name {} already exists", name);
            });
        let output = output::PyOutput::new(name);
        output
    }

    #[pyo3(name = "write_to")]
    fn write_to(&self, path: PathBuf) -> () {
        // output this instance as a json file
        let self_serializable = writer_schema::Procedure::from(self.clone());
        let json = serde_json::to_string(&self_serializable).unwrap();
        std::fs::write(path, json).unwrap();
    }
}
