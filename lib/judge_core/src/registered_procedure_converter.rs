use crate::procedure::{registered::Execution, *};
use std::collections::HashMap;
use crate::identifiers::{DepId, RuntimeId, ResourceId};
pub fn convert(
    registered_procedure: &registered::Procedure,
    runtime_text_contents: &std::collections::HashMap<String, String>,
) -> Result<(runtime::Procedure, HashMap<RuntimeId, DepId>), ConversionError> {
    let mut runtime_id_to_dep_id = HashMap::new();
    let mut dep_id_to_runtime_id = HashMap::new();
    for text in registered_procedure.texts.iter() {
        let runtime_id = RuntimeId::new();
        let dep_id = text.dep_id.clone();
        runtime_id_to_dep_id.insert(runtime_id.clone(), dep_id.clone());
        dep_id_to_runtime_id.insert(dep_id.clone(), runtime_id.clone());
    }
    for empty_directory in registered_procedure.empty_directories.iter() {
        let runtime_id = RuntimeId::new();
        let dep_id = empty_directory.dep_id.clone();
        runtime_id_to_dep_id.insert(runtime_id.clone(), dep_id.clone());
        dep_id_to_runtime_id.insert(dep_id.clone(), runtime_id.clone());
    }
    for runtime_text in registered_procedure.runtime_texts.iter() {
        let runtime_id = RuntimeId::new();
        let dep_id = runtime_text.dep_id.clone();
        runtime_id_to_dep_id.insert(runtime_id.clone(), dep_id.clone());
        dep_id_to_runtime_id.insert(dep_id.clone(), runtime_id.clone());
    }
    for execution in registered_procedure.executions.iter() {
        let runtime_id = RuntimeId::new();
        let dep_id = execution.dep_id.clone();
        runtime_id_to_dep_id.insert(runtime_id.clone(), dep_id.clone());
        dep_id_to_runtime_id.insert(dep_id.clone(), runtime_id.clone());
    }
    let mut texts = Vec::new();
    let mut runtime_texts = Vec::new();
    let mut empty_directories = Vec::new();
    let mut executions = Vec::new();
    for text in registered_procedure.texts.iter() {
        let runtime_id = dep_id_to_runtime_id
            .get(&text.dep_id)
            .ok_or(ConversionError::InternalError("Text dep_id not found".to_string()))?
            .clone();
        texts.push(runtime::Text {
            resource_id: text.resource_id.clone(),
            runtime_id,
        });
    }
    for empty_directory in registered_procedure.empty_directories.iter() {
        let runtime_id = dep_id_to_runtime_id
            .get(&empty_directory.dep_id)
            .ok_or(ConversionError::InternalError("EmptyDirectory dep_id not found".to_string()))?
            .clone();
        empty_directories.push(runtime::EmptyDirectory {
            runtime_id,
        });
    }
    for runtime_text in registered_procedure.runtime_texts.iter() {
        let runtime_id = dep_id_to_runtime_id
            .get(&runtime_text.dep_id)
            .ok_or(ConversionError::InternalError("RuntimeText dep_id not found".to_string()))?
            .clone();
        let content = runtime_text_contents
            .get(&runtime_text.label)
            .ok_or(ConversionError::RuntimeTextNotFound(runtime_text.label.clone()))?
            .clone();
        runtime_texts.push(runtime::RuntimeText {
            content,
            runtime_id,
        });
    }
    for execution in registered_procedure.executions.iter() {
        let runtime_id = dep_id_to_runtime_id
            .get(&execution.dep_id)
            .ok_or(ConversionError::InternalError("Execution runtime_id not found".to_string()))?
            .clone();
        let mut depends_on = Vec::new();
        for dep in execution.depends_on.iter() {
            let runtime_id = dep_id_to_runtime_id
                .get(&dep.dep_id)
                .ok_or(ConversionError::InternalError("DependsOn runtime_id not found".to_string()))?
                .clone();
            depends_on.push(runtime::DependsOn {
                runtime_id,
                envvar_name: dep.envvar_name.clone(),
            });
        }
        executions.push(runtime::Execution {
            depends_on,
            runtime_id,
        });
    }
    let procedure = runtime::Procedure {
        texts,
        runtime_texts,
        empty_directories,
        executions,
    };
    Ok((procedure, runtime_id_to_dep_id))
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ConversionError {
    #[error("Internal error while converting a registered procedure: {0}")]
    InternalError(String),
    #[error("Runtime text not found: {0}")]
    RuntimeTextNotFound(String),
}