use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEmptyDirectory {
    pub name: String,
}

impl From<PyEmptyDirectory> for SchemaEmptyDirectory {
    fn from(py_empty_directory: PyEmptyDirectory) -> Self {
        SchemaEmptyDirectory {
            name: py_empty_directory.name,
        }
    }
}
