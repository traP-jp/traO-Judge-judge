use crate::models::*;
use core::panic;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    resources: HashMap<String, resource_kind::SchemaResourceKind>,
    executions: HashMap<String, execution::SchemaExecution>,
    scripts: HashMap<String, script::SchemaScript>,
}

#[pymethods]
impl Environment {
    #[new]
    fn new() -> Self {
        Environment {
            resources: HashMap::new(),
            executions: HashMap::new(),
            scripts: HashMap::new(),
        }
    }

    #[pyo3(name = "add_resource")]
    fn add_resource(&mut self, resource: resource_kind::PyResourceKind) -> output::PyOutput {
        let schema_resource = resource_kind::SchemaResourceKind::from(resource);
        let name = schema_resource.name().clone();
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
        let schema_script = script::SchemaScript::from(script);
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
                let schema_dep = dependency::SchemaDependency::from(dep.clone());
                schema_dep
            })
            .collect::<Vec<_>>();
        let schema_execution =
            execution::SchemaExecution::new(execution.name, script_name, dependencies);
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
        let json = serde_json::to_string(self).unwrap();
        std::fs::write(path, json).unwrap();
    }
}
