pub mod common;
pub mod v0;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;

#[pymodule(name = "util")]
/// This module provides utilities for traOJudge v1 spec.
fn util_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    v0::v0_module(m)?;
    common::common_module(m)?;
    m.add_function(wrap_pyfunction!(v0::command::get_language_info, m)?)?;
    Ok(())
}

#[gen_stub_pyfunction(module = "traopy_util.util")]
#[pyfunction]
/// Marker to ensure pyo3_stub_gen imports nested submodules
pub fn _marker_fn() -> PyResult<()> {
    Ok(())
}

pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
