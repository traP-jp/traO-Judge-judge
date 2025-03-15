pub mod command;
pub mod input;
pub mod judge_status;
pub mod output;
use pyo3::prelude::*;

#[pymodule(name = "v1")]
/// This module provides utilities for traOJudge v0 spec.
pub fn traopy_util_v1(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<judge_status::JudgeStatus>()?;
    m.add_class::<judge_status::JudgeStatusPriority>()?;
    m.add_function(wrap_pyfunction!(judge_status::merge_judge_status, m)?)?;
    m.add_function(wrap_pyfunction!(output::displayable_result, m)?)?;
    m.add_function(wrap_pyfunction!(output::hidden_result, m)?)?;
    m.add_class::<input::ExecutionMetadata>()?;
    m.add_class::<command::Output>()?;
    m.add_function(wrap_pyfunction!(command::build, m)?)?;
    m.add_function(wrap_pyfunction!(command::run, m)?)?;
    Ok(())
}
