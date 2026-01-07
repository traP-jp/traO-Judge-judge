use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub resources: Vec<ResourceKind>,
    pub executions: Vec<Execution>,
    pub scripts: Vec<Text>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub ref_to: String,
    pub envvar_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyDirectory {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub name: String,
    pub script_name: String,
    pub dependencies: Vec<Dependency>,
    pub time_reserved_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceKind {
    EmptyDirectory(EmptyDirectory),
    RuntimeTextFile(RuntimeText),
    TextFile(Text),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeText {
    pub name: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub name: String,
    pub sha256: String,
}

pub trait UploadedText {
    fn get_sha256(&self) -> String;
}
