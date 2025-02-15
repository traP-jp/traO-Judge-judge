use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyScript {
    pub name: String,
    pub path: PathBuf,
}

#[pymethods]
impl PyScript {
    #[new]
    fn new(name: String, path: PathBuf) -> Self {
        PyScript { name, path }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaScript {
    pub content: String,
    pub name: String,
}

impl From<PyScript> for SchemaScript {
    fn from(py_script: PyScript) -> Self {
        let content = std::fs::read_to_string(&py_script.path).unwrap();
        SchemaScript {
            content,
            name: py_script.name,
        }
    }
}
