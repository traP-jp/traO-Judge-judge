use crate::identifiers::{DepId, ResourceId};

pub struct Procedure {
    pub runtime_texts: Vec<RuntimeText>,
    pub texts: Vec<Text>,
    pub empty_directories: Vec<EmptyDirectory>,
    pub executions: Vec<Execution>,
}

pub struct RuntimeText {
    pub name: String,
    pub dep_id: DepId,
}

pub struct Text {
    pub resource_id: ResourceId,
    pub dep_id: DepId,
}

pub struct EmptyDirectory {
    pub dep_id: DepId,
}

pub struct Execution {
    pub script: String,
    pub depends_on: Vec<DependsOn>,
    pub dep_id: DepId,
    pub priority: i32,
}

pub struct DependsOn {
    pub dep_id: DepId,
    pub envvar_name: String,
}
