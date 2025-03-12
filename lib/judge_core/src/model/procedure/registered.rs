use super::super::identifiers::{DepId, ResourceId};

#[derive(Debug, Clone)]
pub struct Procedure {
    pub runtime_texts: Vec<RuntimeText>,
    pub texts: Vec<Text>,
    pub empty_directories: Vec<EmptyDirectory>,
    pub executions: Vec<Execution>,
}

#[derive(Debug, Clone)]
pub struct RuntimeText {
    pub label: String,
    pub dep_id: DepId,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub resource_id: ResourceId,
    pub dep_id: DepId,
}

#[derive(Debug, Clone)]
pub struct EmptyDirectory {
    pub dep_id: DepId,
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub dependency: Vec<Dependency>,
    pub dep_id: DepId,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub dep_id: DepId,
    pub envvar_name: String,
}
