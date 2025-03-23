pub mod command;
pub mod exec_with_stats;
pub mod output;
use pyo3::prelude::*;

#[pymodule(name = "v0")]
/// This module provides utilities for traOJudge v0 spec.
pub fn v0_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<command::Library>()?;
    m.add_class::<command::Language>()?;
    m.add_wrapped(pyo3::wrap_pyfunction!(command::get_language_info))?;
    m.add_wrapped(pyo3::wrap_pyfunction!(exec_with_stats::exec_with_stats))?;
    m.add_wrapped(pyo3::wrap_pyfunction!(output::jsonify_displayable_output))?;
    m.add_wrapped(pyo3::wrap_pyfunction!(output::jsonify_hidden_output))?;
    m.add_class::<output::JudgeStatus>()?;
    Ok(())
}
