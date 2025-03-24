pub mod read_files;
use pyo3::prelude::*;

pub fn common_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub_mod = PyModule::new(py, "common")?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(read_files::read_file_with_envvar, &sub_mod)?)?;
    Ok(())
}
