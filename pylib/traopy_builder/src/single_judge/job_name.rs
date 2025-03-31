use judge_core::constant::job_name;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

pub fn job_name_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub = PyModule::new(py, "job_name")?;
    sub.add_function(wrap_pyfunction!(compile_phase, &sub)?)?;
    sub.add_function(wrap_pyfunction!(test_phase, &sub)?)?;
    sub.add_function(wrap_pyfunction!(summary_phase, &sub)?)?;
    sub.add_function(wrap_pyfunction!(test_input_file, &sub)?)?;
    sub.add_function(wrap_pyfunction!(test_expected_file, &sub)?)?;
    parent.add_submodule(&sub)?;
    Ok(())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.job_name")]
pub fn compile_phase() -> PyResult<String> {
    Ok(job_name::COMPILE_PHASE.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.job_name")]
pub fn test_phase(core_name: String) -> PyResult<String> {
    Ok(job_name::test_phase_execution_job_name(&core_name))
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.job_name")]
pub fn summary_phase() -> PyResult<String> {
    Ok(job_name::SUMMARY_PHASE.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.job_name")]
pub fn test_input_file(core_name: String) -> PyResult<String> {
    Ok(job_name::v0_features::testcase_input_name(
        core_name.as_str(),
    ))
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.job_name")]
pub fn test_expected_file(core_name: String) -> PyResult<String> {
    Ok(job_name::v0_features::testcase_expected_name(
        core_name.as_str(),
    ))
}
