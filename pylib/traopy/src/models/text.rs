use judge_core::model::procedure::writer_schema::Text;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::path::PathBuf;

/// Text object to be placed in the execution environment.
///
/// Contents of Text must be static.
#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "Text")]
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
