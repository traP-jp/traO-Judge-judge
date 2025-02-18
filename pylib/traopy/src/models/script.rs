use judge_core::procedure::writer_schema::Script;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyScript {
    pub name: String,
    pub path: PathBuf,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyScript {
    #[new]
    fn new(name: String, path: PathBuf) -> Self {
        PyScript { name, path }
    }
}

impl From<PyScript> for Script {
    fn from(py_script: PyScript) -> Self {
        let content = std::fs::read_to_string(&py_script.path).unwrap();
        Script {
            content,
            name: py_script.name,
        }
    }
}
