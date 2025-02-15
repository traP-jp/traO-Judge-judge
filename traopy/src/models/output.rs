use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyOutput {
    pub(crate) name: String,
}

impl PyOutput {
    pub(crate) fn new(name: String) -> Self {
        PyOutput {
            name,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyScriptOutput {
    pub(crate) name: String,
}

impl PyScriptOutput {
    pub(crate) fn new(name: String) -> Self {
        PyScriptOutput {
            name,
        }
    }
}