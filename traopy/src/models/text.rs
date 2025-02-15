use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyText {
    pub name: String,
    pub path: PathBuf,
}

#[pymethods]
impl PyText {
    #[new]
    pub fn new(name: String, path: PathBuf) -> Self {
        PyText {
            name,
            path,
        }
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