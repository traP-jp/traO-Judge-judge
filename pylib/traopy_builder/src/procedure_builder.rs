use crate::models::*;
use judge_core::logic::procedure_builder;
use judge_core::model::procedure::writer_schema;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::path::PathBuf;

/// ProcedureBuilder object to build a procedure.
#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "ProcedureBuilder")]
#[derive(Debug, Clone)]
pub struct PyProcedureBuilder {
    builder: procedure_builder::ProcedureBuilder,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyProcedureBuilder {
    #[new]
    fn new() -> Self {
        PyProcedureBuilder {
            builder: procedure_builder::ProcedureBuilder::new(),
        }
    }

    /// Add a resource to the procedure.
    #[pyo3(name = "add_resource")]
    fn add_resource(&mut self, resource: resource_kind::PyResourceKind) -> output::PyOutput {
        let schema_resource = writer_schema::ResourceKind::from(resource);
        let name = self.builder.add_resource(schema_resource).unwrap();
        let output = output::PyOutput::new(name);
        output
    }

    /// Add a script to the procedure.
    #[pyo3(name = "add_script")]
    fn add_script(&mut self, script: text::PyText) -> output::PyScriptOutput {
        let schema_script = writer_schema::Text::from(script);
        let name = self.builder.add_script(schema_script).unwrap();
        let output = output::PyScriptOutput::new(name);
        output
    }

    /// Add an execution to the procedure.
    #[pyo3(name = "add_execution")]
    fn add_execution(&mut self, execution: execution::PyExecution) -> output::PyOutput {
        let script_name = execution.script.name.clone();
        let dependencies = execution
            .dependencies
            .iter()
            .map(|dep: &dependency::PyDependency| {
                let schema_dep = writer_schema::Dependency::from(dep.clone());
                schema_dep
            })
            .collect::<Vec<_>>();
        let schema_execution = execution::new_execution(execution.name, script_name, dependencies);
        let name = self.builder.add_execution(schema_execution).unwrap();
        let output = output::PyOutput::new(name);
        output
    }

    /// Export the procedure to a json file.
    #[pyo3(name = "write_to")]
    fn write_to(&self, path: PathBuf) -> () {
        // output this instance as a json file
        let serializable = self.builder.get_procedure();
        let json = serde_json::to_string(&serializable).unwrap();
        std::fs::write(path, json).unwrap();
    }
}

impl PyProcedureBuilder {
    pub fn get_schema_procedure(&self) -> writer_schema::Procedure {
        self.builder.get_procedure()
    }
}
