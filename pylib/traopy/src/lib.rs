use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
pub mod local_judge;
pub mod models;
pub mod procedure_builder;

#[pymodule]
fn lowlevel(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<procedure_builder::PyProcedureBuilder>()?;
    m.add_class::<models::dependency::PyDependency>()?;
    m.add_class::<models::empty_directory::PyEmptyDirectory>()?;
    m.add_class::<models::execution::PyExecution>()?;
    m.add_class::<models::runtime_text::PyRuntimeText>()?;
    m.add_class::<models::output::PyOutput>()?;
    m.add_class::<models::output::PyScriptOutput>()?;
    m.add_class::<models::resource_kind::PyResourceKind>()?;
    m.add_class::<models::text::PyText>()?;
    m.add_class::<local_judge::LocalJudge>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
