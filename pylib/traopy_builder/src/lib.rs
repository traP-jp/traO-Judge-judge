use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
pub mod builder;

#[pymodule(name = "lowlevel")]
fn lowlevel(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<builder::Builder>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
