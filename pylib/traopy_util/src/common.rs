pub mod checker;
pub mod read_files;
use pyo3::prelude::*;

pub fn common_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub_mod = PyModule::new(py, "common")?;
    // file reading utilities
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        read_files::read_file_with_envvar,
        &sub_mod
    )?)?;
    // checker utilities
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        checker::normal_judge_checker,
        &sub_mod
    )?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        checker::parse_whitespace_and_newline,
        &sub_mod
    )?)?;
    Ok(())
}
