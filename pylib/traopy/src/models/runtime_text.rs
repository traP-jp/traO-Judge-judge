use judge_core::procedure::writer_schema::RuntimeText;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

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

impl From<PyRuntimeText> for RuntimeText {
    fn from(py_runtime_text: PyRuntimeText) -> Self {
        RuntimeText {
            name: py_runtime_text.name,
        }
    }
}
