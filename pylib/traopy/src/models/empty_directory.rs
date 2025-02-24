use judge_core::procedure::writer_schema::EmptyDirectory;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "EmptyDirectory")]
#[derive(Debug, Clone)]
pub struct PyEmptyDirectory {
    pub name: String,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyEmptyDirectory {
    #[new]
    pub fn new(name: String) -> Self {
        PyEmptyDirectory { name }
    }
}

impl From<PyEmptyDirectory> for EmptyDirectory {
    fn from(py_empty_directory: PyEmptyDirectory) -> Self {
        EmptyDirectory {
            name: py_empty_directory.name,
        }
    }
}
