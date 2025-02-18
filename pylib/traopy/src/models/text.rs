use judge_core::procedure::writer_schema::Text;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
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

impl From<PyText> for Text {
    fn from(py_text: PyText) -> Self {
        let content = std::fs::read_to_string(&py_text.path).unwrap();
        Text {
            name: py_text.name,
            content,
        }
    }
}
