use judge_core::constant::label;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

pub fn label_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub = PyModule::new(py, "label")?;
    sub.add_function(wrap_pyfunction!(submission_source, &sub)?)?;
    sub.add_function(wrap_pyfunction!(language_tag, &sub)?)?;
    sub.add_function(wrap_pyfunction!(time_limit_ms, &sub)?)?;
    sub.add_function(wrap_pyfunction!(memory_limit_kib, &sub)?)?;
    parent.add_submodule(&sub)?;
    Ok(())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.label")]
pub fn submission_source() -> PyResult<String> {
    Ok(label::single_judge::SUBMISSION_SOURCE.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.label")]
pub fn language_tag() -> PyResult<String> {
    Ok(label::single_judge::LANGUAGE_TAG.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.label")]
pub fn time_limit_ms() -> PyResult<String> {
    Ok(label::single_judge::TIME_LIMIT_MS.to_string())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge.label")]
pub fn memory_limit_kib() -> PyResult<String> {
    Ok(label::single_judge::MEMORY_LIMIT_KIB.to_string())
}
