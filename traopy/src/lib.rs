use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
pub mod environment;
pub mod models;

#[pymodule]
fn lowlevel(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<environment::Environment>()?;
    m.add_class::<models::dependency::PyDependency>()?;
    m.add_class::<models::empty_directory::PyEmptyDirectory>()?;
    m.add_class::<models::execution::PyExecution>()?;
    m.add_class::<models::runtime_text::PyRuntimeText>()?;
    m.add_class::<models::output::PyOutput>()?;
    m.add_class::<models::output::PyScriptOutput>()?;
    m.add_class::<models::resource_kind::PyResourceKind>()?;
    m.add_class::<models::script::PyScript>()?;
    m.add_class::<models::text::PyText>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
