use domain::model::procedure::{
    Dependency, EmptyDirectory, Execution, Procedure, RuntimeText, Text,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProcedureRow {
    pub procedure: sqlx::types::Json<ProcedureJson>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcedureJson {
    pub runtime_texts: Vec<RuntimeTextJson>,
    pub texts: Vec<TextJson>,
    pub empty_directories: Vec<EmptyDirectoryJson>,
    pub executions: Vec<ExecutionJson>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuntimeTextJson {
    pub label: String,
    pub dep_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextJson {
    pub resource_id: Uuid,
    pub dep_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmptyDirectoryJson {
    pub dep_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionJson {
    pub dependencies: Vec<DependencyJson>,
    pub dep_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DependencyJson {
    pub dep_id: Uuid,
    pub envvar_name: String,
}

impl From<ProcedureJson> for Procedure {
    fn from(val: ProcedureJson) -> Self {
        Procedure {
            runtime_texts: val.runtime_texts.into_iter().map(Into::into).collect(),
            texts: val.texts.into_iter().map(Into::into).collect(),
            empty_directories: val.empty_directories.into_iter().map(Into::into).collect(),
            executions: val.executions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Procedure> for ProcedureJson {
    fn from(val: Procedure) -> Self {
        ProcedureJson {
            runtime_texts: val.runtime_texts.into_iter().map(Into::into).collect(),
            texts: val.texts.into_iter().map(Into::into).collect(),
            empty_directories: val.empty_directories.into_iter().map(Into::into).collect(),
            executions: val.executions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<RuntimeTextJson> for RuntimeText {
    fn from(val: RuntimeTextJson) -> Self {
        RuntimeText {
            label: val.label,
            dep_id: val.dep_id,
        }
    }
}

impl From<RuntimeText> for RuntimeTextJson {
    fn from(val: RuntimeText) -> Self {
        RuntimeTextJson {
            label: val.label,
            dep_id: val.dep_id,
        }
    }
}

impl From<TextJson> for Text {
    fn from(val: TextJson) -> Self {
        Text {
            resource_id: val.resource_id,
            dep_id: val.dep_id,
        }
    }
}

impl From<Text> for TextJson {
    fn from(val: Text) -> Self {
        TextJson {
            resource_id: val.resource_id,
            dep_id: val.dep_id,
        }
    }
}

impl From<EmptyDirectoryJson> for EmptyDirectory {
    fn from(val: EmptyDirectoryJson) -> Self {
        EmptyDirectory { dep_id: val.dep_id }
    }
}

impl From<EmptyDirectory> for EmptyDirectoryJson {
    fn from(val: EmptyDirectory) -> Self {
        EmptyDirectoryJson { dep_id: val.dep_id }
    }
}

impl From<ExecutionJson> for Execution {
    fn from(val: ExecutionJson) -> Self {
        Execution {
            dependencies: val.dependencies.into_iter().map(Into::into).collect(),
            dep_id: val.dep_id,
        }
    }
}

impl From<Execution> for ExecutionJson {
    fn from(val: Execution) -> Self {
        ExecutionJson {
            dependencies: val.dependencies.into_iter().map(Into::into).collect(),
            dep_id: val.dep_id,
        }
    }
}

impl From<DependencyJson> for Dependency {
    fn from(val: DependencyJson) -> Self {
        Dependency {
            dep_id: val.dep_id,
            envvar_name: val.envvar_name,
        }
    }
}

impl From<Dependency> for DependencyJson {
    fn from(val: Dependency) -> Self {
        DependencyJson {
            dep_id: val.dep_id,
            envvar_name: val.envvar_name,
        }
    }
}
