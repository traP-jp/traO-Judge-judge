use super::{empty_directory::*, runtime_text::*, text::*};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug, Clone)]
pub enum PyResourceKind {
    EmptyDirectory(PyEmptyDirectory),
    OnetimeTextFile(PyOnetimeText),
    TextFile(PyText),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaResourceKind {
    EmptyDirectory(SchemaEmptyDirectory),
    OnetimeTextFile(SchemaOnetimeText),
    TextFile(SchemaText),
}

impl SchemaResourceKind {
    pub fn name(&self) -> String {
        match self {
            SchemaResourceKind::EmptyDirectory(empty_directory) => empty_directory.name.clone(),
            SchemaResourceKind::OnetimeTextFile(runtime_text) => runtime_text.name.clone(),
            SchemaResourceKind::TextFile(text) => text.name.clone(),
        }
    }
}

impl From<PyResourceKind> for SchemaResourceKind {
    fn from(py_resource_kind: PyResourceKind) -> Self {
        match py_resource_kind {
            PyResourceKind::EmptyDirectory(py_empty_directory) => {
                SchemaResourceKind::EmptyDirectory(py_empty_directory.into())
            }
            PyResourceKind::OnetimeTextFile(py_runtime_text) => {
                SchemaResourceKind::OnetimeTextFile(py_runtime_text.into())
            }
            PyResourceKind::TextFile(py_text) => SchemaResourceKind::TextFile(py_text.into()),
        }
    }
}
