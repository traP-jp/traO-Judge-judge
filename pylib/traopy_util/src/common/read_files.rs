use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.common")]
pub fn read_file_with_envvar(envvar: String) -> PyResult<String> {
    let envvar = std::env::var(envvar.clone()).map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Environment variable not found: {}",
            envvar
        ))
    })?;
    let file_path = std::path::PathBuf::from(envvar.clone());
    let content = std::fs::read_to_string(file_path).map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to read file: {}", envvar))
    })?;
    Ok(content)
}
