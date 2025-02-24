use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// Output object of each job.
///
/// Executions can depend on Output by passing Output object to `depends_on` field of Execution.
#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "Output")]
#[derive(Debug, Clone)]
pub struct PyOutput {
    pub(crate) name: String,
}

impl PyOutput {
    pub(crate) fn new(name: String) -> Self {
        PyOutput { name }
    }
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(name = "ScriptOutput")]
pub struct PyScriptOutput {
    pub(crate) name: String,
}

impl PyScriptOutput {
    pub(crate) fn new(name: String) -> Self {
        PyScriptOutput { name }
    }
}
