use super::{dependency::*, output::PyScriptOutput};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExecution {
    pub name: String,
    pub script: PyScriptOutput,
    pub depends_on: Vec<PyDependency>,
}

#[gen_stub_pymethods]
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
