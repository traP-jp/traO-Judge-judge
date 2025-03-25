pub mod command;
pub mod constant;
pub mod exec_with_stats;
pub mod output;
use pyo3::prelude::*;

/// This module provides utilities for traOJudge v0 spec.
pub fn v0_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub_mod = PyModule::new(py, "v0")?;
    // command utilities
    sub_mod.add_class::<command::Library>()?;
    sub_mod.add_class::<command::Language>()?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        command::get_language_info,
        &sub_mod
    )?)?;
    // execution utilities
    sub_mod.add_class::<exec_with_stats::ExecStats>()?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        exec_with_stats::exec_with_stats,
        &sub_mod
    )?)?;
    // output support utilities
    sub_mod.add_class::<output::JudgeStatus>()?;
    sub_mod.add_class::<output::ExecutionResult>()?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        output::jsonify_displayable_output,
        &sub_mod
    )?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        output::jsonify_hidden_output,
        &sub_mod
    )?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(output::dejsonify_output, &sub_mod)?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        output::merge_judge_status,
        &sub_mod
    )?)?;
    // constant utilities
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        constant::build_source_envvar,
        &sub_mod
    )?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        constant::build_output_envvar,
        &sub_mod
    )?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        constant::exec_job_outcome_path_envvar,
        &sub_mod
    )?)?;
    sub_mod.add_function(pyo3::wrap_pyfunction!(
        constant::build_tempdir_envvar,
        &sub_mod
    )?)?;
    parent.add_submodule(&sub_mod)?;
    Ok(())
}
