#![allow(unused_variables)]

use crate::model::{
    problem_registry::*,
    dep_name_repository::*,
    procedure::{writer_schema::*, *},
    *,
};
use std::collections::HashMap;

pub async fn register<
    PRServer: ProblemRegistryServer,
    DNRepo: DepNameRepository,
>(
    problem: writer_schema::Procedure,
    pr_server: PRServer,
    dn_repo: DNRepo,
) -> Result<registered::Procedure, RegistrationError> {
    let (procedure, content_to_id, name_to_id) = transpile_inner(problem)?;
    dn_repo.insert_many(name_to_id)
        .await
        .map_err(|e| RegistrationError::InternalError(e.to_string()))?;
    pr_server.register_many(content_to_id)
        .await
        .map_err(|e| RegistrationError::InternalError(e.to_string()))?;
    Ok(procedure)
}

fn transpile_inner(
    problem: writer_schema::Procedure,
) -> Result<
    (
        registered::Procedure,
        HashMap<identifiers::ResourceId, String>,
        HashMap<identifiers::DepId, String>,
    ),
    RegistrationError,
> {
    let mut name_to_id = HashMap::new();
    for resource in problem.resources.iter() {
        let name = match resource {
            ResourceKind::TextFile(content) => content.name.clone(),
            ResourceKind::EmptyDirectory(empty_dir) => empty_dir.name.clone(),
            ResourceKind::RuntimeTextFile(runtime_text) => runtime_text.name.clone(),
        };
        let id = identifiers::DepId::new();
        name_to_id.insert(name.clone(), id);
    }
    for script in problem.scripts.iter() {
        let name = script.name.clone();
        let id = identifiers::DepId::new();
        name_to_id.insert(name.clone(), id);
    }
    for execution in problem.executions.iter() {
        let name = execution.name.clone();
        let id = identifiers::DepId::new();
        name_to_id.insert(name.clone(), id);
    }
    let mut content_to_id = HashMap::new();
    for resource in problem.resources.iter() {
        if let ResourceKind::TextFile(content) = resource {
            let id = identifiers::ResourceId::new();
            content_to_id.insert(content.content.clone(), id);
        }
    }
    for script in problem.scripts.iter() {
        let id = identifiers::ResourceId::new();
        content_to_id.insert(script.content.clone(), id);
    }
    let mut runtime_texts = Vec::new();
    let mut texts = Vec::new();
    let mut empty_directories = Vec::new();
    for resource in problem.resources.iter() {
        match resource {
            ResourceKind::TextFile(content) => {
                let dep_id = name_to_id
                    .get(&content.name)
                    .ok_or(RegistrationError::InvalidSchema(
                        "TextFile name not found".to_string(),
                    ))?
                    .clone();
                let text = registered::Text {
                    resource_id: content_to_id
                        .get(&content.content)
                        .ok_or(RegistrationError::InvalidSchema(
                            "TextFile content not found".to_string(),
                        ))?
                        .clone(),
                    dep_id: dep_id.clone(),
                };
                texts.push(text);
            }
            ResourceKind::EmptyDirectory(empty_dir) => {
                let dep_id = name_to_id
                    .get(&empty_dir.name)
                    .ok_or(RegistrationError::InvalidSchema(
                        "EmptyDirectory name not found".to_string(),
                    ))?
                    .clone();
                let empty_directory = registered::EmptyDirectory {
                    dep_id: dep_id.clone(),
                };
                empty_directories.push(empty_directory);
            }
            ResourceKind::RuntimeTextFile(runtime_text) => {
                let dep_id = name_to_id
                    .get(&runtime_text.name)
                    .ok_or(RegistrationError::InvalidSchema(
                        "RuntimeText name not found".to_string(),
                    ))?
                    .clone();
                let runtime_text = registered::RuntimeText {
                    label: runtime_text.label.clone(),
                    dep_id: dep_id.clone(),
                };
                runtime_texts.push(runtime_text);
            }
        }
    }
    let mut executions = Vec::new();
    for execution in problem.executions.iter() {
        let script_id = name_to_id
            .get(&execution.script_name)
            .ok_or(RegistrationError::InvalidSchema(
                "Execution script name not found".to_string(),
            ))?
            .clone();
        let mut dependencies = Vec::new();
        for dep in execution.dependencies.iter() {
            let dep_id = name_to_id
                .get(&dep.ref_to)
                .ok_or(RegistrationError::InvalidSchema(
                    "Dependency name not found".to_string(),
                ))?
                .clone();
            let dependency = registered::Dependency {
                dep_id: dep_id.clone(),
                envvar_name: dep.envvar_name.clone(),
            };
            dependencies.push(dependency);
        }
        dependencies.push(registered::Dependency {
            dep_id: script_id.clone(),
            envvar_name: "SCRIPT".to_string(),
        });
        let dep_id = name_to_id
            .get(&execution.name)
            .ok_or(RegistrationError::InvalidSchema(
                "Execution name not found".to_string(),
            ))?
            .clone();
        let execution = registered::Execution {
            dependencies,
            dep_id: dep_id,
        };
        executions.push(execution);
    }
    for script in problem.scripts.iter() {
        let name = script.name.clone();
        let dep_id = name_to_id
            .get(&name)
            .ok_or(RegistrationError::InvalidSchema(
                "Script name not found".to_string(),
            ))?
            .clone();
        let resource_id = content_to_id
            .get(&script.content)
            .ok_or(RegistrationError::InvalidSchema(
                "Script content not found".to_string(),
            ))?
            .clone();
        let text = registered::Text {
            resource_id,
            dep_id,
        };
        texts.push(text);
    }
    let procedure = registered::Procedure {
        runtime_texts,
        texts,
        empty_directories,
        executions,
    };
    Ok((
        procedure,
        content_to_id
            .iter()
            .map(|(k, v)| (v.clone(), k.clone()))
            .collect(),
        name_to_id
            .iter()
            .map(|(k, v)| (v.clone(), k.clone()))
            .collect(),
    ))
}
