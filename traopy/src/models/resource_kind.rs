use super::{empty_directory::*, onetime_text::*, text::*};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone)]
pub enum PyResourceKind {
    EmptyDirectory(PyEmptyDirectory),
    OnetimeTextFile(PyOnetimeText),
    TextFile(PyText),
}

#[pymethods]
impl PyResourceKind {
    #[staticmethod]
    fn new_empty_directory(empty_directory: PyEmptyDirectory) -> Self {
        PyResourceKind::EmptyDirectory(empty_directory)
    }

    #[staticmethod]
    fn new_onetime_text_file(onetime_text: PyOnetimeText) -> Self {
        PyResourceKind::OnetimeTextFile(onetime_text)
    }

    #[staticmethod]
    fn new_text_file(text: PyText) -> Self {
        PyResourceKind::TextFile(text)
    }
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
            SchemaResourceKind::OnetimeTextFile(onetime_text) => onetime_text.name.clone(),
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
            PyResourceKind::OnetimeTextFile(py_onetime_text) => {
                SchemaResourceKind::OnetimeTextFile(py_onetime_text.into())
            }
            PyResourceKind::TextFile(py_text) => SchemaResourceKind::TextFile(py_text.into()),
        }
    }
}
