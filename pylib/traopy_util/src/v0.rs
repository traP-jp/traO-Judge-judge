pub mod input;
pub mod output;
pub mod command;
use pyo3::prelude::*;

#[pymodule(name = "v1")]
/// This module provides utilities for traOJudge v0 spec.
pub fn traopy_util_v1(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<output::JudgeStatus>()?;
    m.add_function(wrap_pyfunction!(output::displayable_result, m)?)?;
    m.add_function(wrap_pyfunction!(output::hidden_result, m)?)?;
    m.add_class::<input::ExecutionMetadata>()?;
    m.add_class::<command::Output>()?;
    m.add_function(wrap_pyfunction!(command::build, m)?)?;
    m.add_function(wrap_pyfunction!(command::run, m)?)?;
    Ok(())
}
