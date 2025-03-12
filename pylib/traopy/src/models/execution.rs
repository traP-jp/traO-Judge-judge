use super::{dependency::*, output::PyScriptOutput};
use judge_core::model::procedure::writer_schema::{Dependency, Execution};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// Execution object to be executed
///
/// Script will be run with paths as environment variables specified in `dependency`.
#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "Execution")]
#[derive(Debug, Clone)]
pub struct PyExecution {
    pub name: String,
    pub script: PyScriptOutput,
    pub dependency: Vec<PyDependency>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyExecution {
    #[new]
    pub fn new(name: String, script: PyScriptOutput, dependency: Vec<PyDependency>) -> Self {
        PyExecution {
            name,
            script,
            dependency,
        }
    }
}

pub fn new_execution(name: String, script_id: String, dependency: Vec<Dependency>) -> Execution {
    Execution {
        name,
        script_name: script_id,
        dependency,
    }
}
