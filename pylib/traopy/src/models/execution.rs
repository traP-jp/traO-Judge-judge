use super::{dependency::*, output::PyScriptOutput};
use judge_core::procedure::writer_schema::{Dependency, Execution};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "Execution")]
#[derive(Debug, Clone)]
pub struct PyExecution {
    pub name: String,
    pub script: PyScriptOutput,
    pub depends_on: Vec<PyDependency>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyExecution {
    #[new]
    pub fn new(name: String, script: PyScriptOutput, depends_on: Vec<PyDependency>) -> Self {
        PyExecution {
            name,
            script,
            depends_on,
        }
    }
}

pub fn new_execution(name: String, script_id: String, depends_on: Vec<Dependency>) -> Execution {
    Execution {
        name,
        script_name: script_id,
        depends_on,
    }
}
