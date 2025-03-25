use judge_core::constant::env_var_exec;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn build_source_envvar() -> PyResult<String> {
    Ok(env_var_exec::BUILD_SOURCE.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn build_output_envvar() -> PyResult<String> {
    Ok(env_var_exec::BUILD_OUTPUT.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn exec_job_outcome_path_envvar() -> PyResult<String> {
    Ok(env_var_exec::OUTPUT_PATH.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn build_tempdir_envvar() -> PyResult<String> {
    Ok(env_var_exec::BUILD_TEMPDIR.to_string())
}
