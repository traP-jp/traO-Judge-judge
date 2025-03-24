pub mod job_name;
pub mod label;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

pub fn single_judge_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let sub = PyModule::new(py, "single_judge")?;
    job_name::job_name_module(&sub)?;
    label::label_module(&sub)?;
    sub.add_function(wrap_pyfunction!(_marker_fn, &sub)?)?;
    parent.add_submodule(&sub)?;
    Ok(())
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_builder.builder.single_judge")]
/// Marker to ensure pyo3_stub_gen imports nested submodules
pub fn _marker_fn() -> PyResult<()> {
    Ok(())
}
