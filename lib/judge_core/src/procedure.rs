use uuid::Uuid;

/// All runtime_id must be unique application-wide for each submission.
pub struct Procedure {
    pub runtime_texts: Vec<RuntimeText>,
    pub texts: Vec<Text>,
    pub empty_directories: Vec<EmptyDirectory>,
    pub executions: Vec<Execution>,
}

pub struct RuntimeText {
    pub content: String,
    pub runtime_id: Uuid,
}

pub struct Text {
    pub resource_id: Uuid,
    pub runtime_id: Uuid,
}

pub struct EmptyDirectory {
    pub runtime_id: Uuid,
}

pub struct Execution {
    pub script: String,
    pub depends_on: Vec<DependsOn>,
    pub runtime_id: Uuid,
    pub priority: i32,
}

pub struct DependsOn {
    pub runtime_id: Uuid,
    pub envvar_name: String,
}
