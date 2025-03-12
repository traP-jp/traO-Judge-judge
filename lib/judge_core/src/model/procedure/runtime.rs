use super::super::identifiers::{ResourceId, RuntimeId};

#[derive(Debug, Clone)]
pub struct Procedure {
    pub runtime_texts: Vec<RuntimeText>,
    pub texts: Vec<Text>,
    pub empty_directories: Vec<EmptyDirectory>,
    pub executions: Vec<Execution>,
}

#[derive(Debug, Clone)]
pub struct RuntimeText {
    pub content: String,
    pub runtime_id: RuntimeId,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub resource_id: ResourceId,
    pub runtime_id: RuntimeId,
}

#[derive(Debug, Clone)]
pub struct EmptyDirectory {
    pub runtime_id: RuntimeId,
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub dependency: Vec<Dependency>,
    pub runtime_id: RuntimeId,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub runtime_id: RuntimeId,
    pub envvar_name: String,
}
