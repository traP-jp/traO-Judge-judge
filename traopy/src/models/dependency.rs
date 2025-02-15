use super::output::PyOutput;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyDependency {
    pub ref_to: PyOutput,
    pub envvar_name: String,
}

#[pymethods]
impl PyDependency {
    #[new]
    fn new(ref_to: PyOutput, envvar_name: String) -> Self {
        PyDependency {
            ref_to,
            envvar_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDependency {
    pub ref_to: String,
    pub envvar_name: String,
}

impl From<PyDependency> for SchemaDependency {
    fn from(py_dependency: PyDependency) -> Self {
        SchemaDependency {
            ref_to: py_dependency.ref_to.name,
            envvar_name: py_dependency.envvar_name,
        }
    }
}
