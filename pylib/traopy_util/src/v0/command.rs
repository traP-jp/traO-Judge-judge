use judge_core::constant::env_var_exec;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};

#[gen_stub_pyclass]
#[pyclass(module = "traopy_util.util.v0")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub version: String,
}

#[gen_stub_pyclass]
#[pyclass(module = "traopy_util.util.v0")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    #[serde(rename = "binName")]
    pub bin_name: String,
    pub compile: String,
    pub run: String,
    pub libraries: Option<Vec<Library>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Language {
    #[getter]
    fn compile(&self) -> String {
        self.compile.clone()
    }
    #[getter]
    fn run(&self) -> String {
        self.run.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Schema {
    pub languages: Vec<Language>,
}

#[pyfunction]
#[gen_stub_pyfunction(module = "traopy_util.util.v0")]
pub fn get_language_info(language_tag: String) -> PyResult<Language> {
    let languages_json_path = std::env::var(env_var_exec::LANGUAGES_JSON).map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Environment variable not found: {}",
            env_var_exec::LANGUAGES_JSON
        ))
    })?;
    let languages_json = std::fs::read_to_string(languages_json_path.clone()).map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to read file: {}",
            languages_json_path
        ))
    })?;
    let schema = serde_json::from_str::<Schema>(languages_json.as_str()).map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Failed to parse languages.json".to_string(),
        )
    })?;
    let language = schema
        .languages
        .into_iter()
        .find(|l| l.name == language_tag)
        .ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Language not found: {}",
                language_tag
            ))
        })?;
    Ok(language)
}
