use judge_core::constant::env_var_exec;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::path::PathBuf;

/// Judge input.
#[pyclass]
#[gen_stub_pyclass]
#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    pub time_limit_ms: Option<f64>,
    pub memory_limit_kib: Option<i64>,
    pub language: Option<String>,
    pub submission_path: PathBuf,
    pub output_path: PathBuf,
}

#[pymethods]
#[gen_stub_pymethods]
impl ExecutionMetadata {
    #[new]
    /// Read the input from environment variables.
    fn new() -> Self {
        let time_limit_ms = std::env::var(env_var_exec::TIME_LIMIT_MS)
            .ok()
            .map(|s| s.parse::<f64>().expect("TIME_LIMIT_MS not a number"));
        let memory_limit_kib = std::env::var(env_var_exec::MEMORY_LIMIT_KIB)
            .ok()
            .map(|s| s.parse::<i64>().expect("MEMORY_LIMIT_KIB not a number"));
        let language = std::env::var(env_var_exec::LANGUAGE)
            .ok()
            .map(|s| s.to_string());
        let submission_path = std::env::var(env_var_exec::SOURCE_PATH)
            .expect("SOURCE_PATH not set")
            .into();
        let output_path = std::env::var(env_var_exec::OUTPUT_PATH)
            .expect("OUTPUT_PATH not set")
            .into();
        Self {
            time_limit_ms,
            memory_limit_kib,
            language,
            submission_path,
            output_path,
        }
    }
}
