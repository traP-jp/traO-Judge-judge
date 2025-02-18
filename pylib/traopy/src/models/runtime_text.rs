use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyRuntimeText {
    pub name: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyRuntimeText {
    #[new]
    pub fn new(name: String) -> Self {
        PyRuntimeText { name }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaRuntimeText {
    pub name: String,
}

impl From<PyRuntimeText> for SchemaRuntimeText {
    fn from(py_runtime_text: PyRuntimeText) -> Self {
        SchemaRuntimeText {
            name: py_runtime_text.name,
        }
    }
}
