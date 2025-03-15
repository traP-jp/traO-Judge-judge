use judge_core::constant::env_var_exec;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Library {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Language {
    pub name: String,
    #[serde(rename = "binName")]
    pub bin_name: String,
    pub compile: String,
    pub run: String,
    pub libraries: Option<Vec<Library>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Schema {
    pub languages: Vec<Language>,
}

fn get_language_info(language_tag: Option<String>) -> Language {
    let languages_json_path = std::env::var(env_var_exec::LANGUAGES_JSON)
        .expect(format!("${} not set", env_var_exec::LANGUAGES_JSON).as_str());
    let languages_json = std::fs::read_to_string(languages_json_path.clone())
        .expect(format!("Failed to read {}", languages_json_path).as_str());
    let schema = serde_json::from_str::<Schema>(languages_json.as_str())
        .expect("Failed to parse languages.json");
    let language_tag = match language_tag {
        Some(tag) => tag,
        None => {
            let language_path = std::env::var(env_var_exec::LANGUAGE)
                .expect(format!("${} not set", env_var_exec::LANGUAGE).as_str());
            let language = std::fs::read_to_string(language_path.clone())
                .expect(format!("Failed to read {}", language_path).as_str());
            language
        }
    };
    let language = schema
        .languages
        .into_iter()
        .find(|l| l.name == language_tag)
        .expect(format!("Language {} not found", language_tag).as_str());
    language
}

/// Process output type
#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
#[gen_stub_pyclass]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Build the source code.
#[pyfunction(signature = (build_source_path, build_output_path, language_tag=None))]
#[gen_stub_pyfunction]
pub fn build(
    build_source_path: PathBuf,
    build_output_path: PathBuf,
    language_tag: Option<String>,
) -> PyResult<Output> {
    let language = get_language_info(language_tag);
    let compile_command = language.compile;
    let envs = vec![
        (
            env_var_exec::BUILD_SOURCE,
            build_source_path.to_str().unwrap(),
        ),
        (
            env_var_exec::BUILD_OUTPUT,
            build_output_path.to_str().unwrap(),
        ),
    ];
    let mut command = std::process::Command::new("sh");
    command.arg("-c").arg(compile_command);
    command.envs(envs);
    let output = command.output().expect("Failed to execute process");
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    let exit_code = output.status.code().unwrap();
    Ok(Output {
        stdout,
        stderr,
        exit_code,
    })
}

/// Run the source code.
#[pyfunction(signature = (build_output_path, language_tag=None))]
#[gen_stub_pyfunction]
pub fn run(build_output_path: PathBuf, language_tag: Option<String>) -> PyResult<Output> {
    let language = get_language_info(language_tag);
    let run_command = language.run;
    let envs = vec![(
        env_var_exec::BUILD_OUTPUT,
        build_output_path.to_str().unwrap(),
    )];
    let mut command = std::process::Command::new("sh");
    command.arg("-c").arg(run_command);
    command.envs(envs);
    let output = command.output().expect("Failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let exit_code = output.status.code().unwrap();
    Ok(Output {
        stdout,
        stderr,
        exit_code,
    })
}
