use judge_core::procedure::writer_schema::RuntimeText;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// RuntimeText object to be placed in the execution environment.
/// 
/// `label`s have corresponding types of resources determined in judge-run time.
/// | label | description |
/// | --- | --- |
/// | `source` | Submission source code |
/// | `time_limit` | Time limit(optional) |
/// | `memory_limit` | Memory limit(optional) |
/// | `language` | Language of the submission(optional) |
#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "RuntimeText")]
#[derive(Debug, Clone)]
pub struct PyRuntimeText {
    pub name: String,
    pub label: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyRuntimeText {
    #[new]
    pub fn new(name: String, label: String) -> Self {
        PyRuntimeText { name, label }
    }
}

impl From<PyRuntimeText> for RuntimeText {
    fn from(py_runtime_text: PyRuntimeText) -> Self {
        RuntimeText {
            name: py_runtime_text.name,
            label: py_runtime_text.label,
        }
    }
}
