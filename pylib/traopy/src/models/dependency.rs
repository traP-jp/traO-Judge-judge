use super::output::PyOutput;
use judge_core::procedure::writer_schema::Dependency;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// Dependency object to refer to previous output.
///
/// Path to the output will be provided as an environment variable named `envvar_name`.
#[gen_stub_pyclass]
#[pyclass]
#[pyo3(name = "Dependency")]
#[derive(Debug, Clone)]
pub struct PyDependency {
    pub ref_to: PyOutput,
    pub envvar_name: String,
}

#[gen_stub_pymethods]
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

impl From<PyDependency> for Dependency {
    fn from(py_dependency: PyDependency) -> Self {
        Dependency {
            ref_to: py_dependency.ref_to.name,
            envvar_name: py_dependency.envvar_name,
        }
    }
}
