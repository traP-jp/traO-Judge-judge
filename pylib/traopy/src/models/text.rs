use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyText {
    pub name: String,
    pub path: PathBuf,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyText {
    #[new]
    pub fn new(name: String, path: PathBuf) -> Self {
        PyText { name, path }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaText {
    pub name: String,
    pub content: String,
}

impl From<PyText> for SchemaText {
    fn from(py_text: PyText) -> Self {
        let content = std::fs::read_to_string(&py_text.path).unwrap();
        SchemaText {
            name: py_text.name,
            content,
        }
    }
}
