use uuid::Uuid;

pub struct Procedure {
    pub runtime_texts: Vec<RuntimeText>,
    pub texts: Vec<Text>,
    pub empty_directories: Vec<EmptyDirectory>,
    pub executions: Vec<Execution>,
}

pub struct RuntimeText {
    pub label: String,
    pub dep_id: Uuid,
}

pub struct Text {
    pub resource_id: Uuid,
    pub dep_id: Uuid,
}

pub struct EmptyDirectory {
    pub dep_id: Uuid,
}

pub struct Execution {
    pub dependencies: Vec<Dependency>,
    pub dep_id: Uuid,
}

pub struct Dependency {
    pub dep_id: Uuid,
    pub envvar_name: String,
}
