use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass]
#[pyclass]
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
pub struct PyScriptOutput {
    pub(crate) name: String,
}

impl PyScriptOutput {
    pub(crate) fn new(name: String) -> Self {
        PyScriptOutput { name }
    }
}
