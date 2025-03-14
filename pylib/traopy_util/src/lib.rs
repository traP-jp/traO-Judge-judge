pub mod v0;
use pyo3::prelude::*;

#[pymodule]
/// This module provides utilities for traOJudge v1 spec.
fn traopy_util_v1(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(pyo3::wrap_pymodule!(v0::traopy_util_v1))?;
    Ok(())
}

pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
