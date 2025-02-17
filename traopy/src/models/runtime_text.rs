use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyOnetimeText {
    pub name: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyOnetimeText {
    #[new]
    pub fn new(name: String) -> Self {
        PyOnetimeText { name }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOnetimeText {
    pub name: String,
}

impl From<PyOnetimeText> for SchemaOnetimeText {
    fn from(py_runtime_text: PyOnetimeText) -> Self {
        SchemaOnetimeText {
            name: py_runtime_text.name,
        }
    }
}
