use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyEmptyDirectory {
    pub name: String,
}

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
