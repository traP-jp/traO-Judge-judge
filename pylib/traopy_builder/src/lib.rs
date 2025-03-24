use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
pub mod builder;
pub mod single_judge;

#[pymodule(name = "builder")]
fn root_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<builder::Builder>()?;
    single_judge::single_judge_module(m)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
