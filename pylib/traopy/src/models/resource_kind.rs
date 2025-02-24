use super::{empty_directory::*, runtime_text::*, text::*};
use judge_core::procedure::writer_schema::ResourceKind;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// Resource object to be placed in the execution environment.
///
/// EmptyDirectory is an directory and TextFile and RuntimeTextFile are files.
#[gen_stub_pyclass_enum]
#[pyclass]
#[pyo3(name = "ResourceKind")]
#[derive(Debug, Clone)]
pub enum PyResourceKind {
    EmptyDirectory(PyEmptyDirectory),
    RuntimeTextFile(PyRuntimeText),
    TextFile(PyText),
}

pub fn resource_name(rc_kind: &ResourceKind) -> String {
    match rc_kind {
        ResourceKind::EmptyDirectory(empty_directory) => empty_directory.name.clone(),
        ResourceKind::RuntimeTextFile(runtime_text) => runtime_text.name.clone(),
        ResourceKind::TextFile(text) => text.name.clone(),
    }
}

impl From<PyResourceKind> for ResourceKind {
    fn from(py_resource_kind: PyResourceKind) -> Self {
        match py_resource_kind {
            PyResourceKind::EmptyDirectory(py_empty_directory) => {
                ResourceKind::EmptyDirectory(py_empty_directory.into())
            }
            PyResourceKind::RuntimeTextFile(py_runtime_text) => {
                ResourceKind::RuntimeTextFile(py_runtime_text.into())
            }
            PyResourceKind::TextFile(py_text) => ResourceKind::TextFile(py_text.into()),
        }
    }
}
