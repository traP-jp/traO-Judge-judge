use crate::identifiers::{ResourceId, RuntimeId};

pub struct Procedure {
    pub runtime_texts: Vec<RuntimeText>,
    pub texts: Vec<Text>,
    pub empty_directories: Vec<EmptyDirectory>,
    pub executions: Vec<Execution>,
}

pub struct RuntimeText {
    pub content: String,
    pub runtime_id: RuntimeId,
}

pub struct Text {
    pub resource_id: ResourceId,
    pub runtime_id: RuntimeId,
}

pub struct EmptyDirectory {
    pub runtime_id: RuntimeId,
}

pub struct Execution {
    pub script: String,
    pub depends_on: Vec<DependsOn>,
    pub runtime_id: RuntimeId,
    pub priority: i32,
}

pub struct DependsOn {
    pub runtime_id: RuntimeId,
    pub envvar_name: String,
}
