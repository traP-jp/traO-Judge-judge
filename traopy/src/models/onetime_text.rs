use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyOnetimeText {
    pub name: String,
}

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
    fn from(py_onetime_text: PyOnetimeText) -> Self {
        SchemaOnetimeText {
            name: py_onetime_text.name,
        }
    }
}
