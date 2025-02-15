use super::{dependency::*, output::PyScriptOutput};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExecution {
    pub name: String,
    pub script: PyScriptOutput,
    pub depends_on: Vec<PyDependency>,
}

#[pymethods]
impl PyExecution {
    #[new]
    pub fn new(name: String, script: PyScriptOutput, depends_on: Vec<PyDependency>) -> Self {
        PyExecution {
            name,
            script,
            depends_on,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaExecution {
    pub name: String,
    pub script_name: String,
    pub depends_on: Vec<SchemaDependency>,
}

impl SchemaExecution {
    pub fn new(name: String, script_id: String, depends_on: Vec<SchemaDependency>) -> Self {
        SchemaExecution {
            name,
            script_name: script_id,
            depends_on,
        }
    }
}
