use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub resources: HashMap<String, ResourceKind>,
    pub executions: HashMap<String, Execution>,
    pub scripts: HashMap<String, Script>,
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
    pub depends_on: Vec<Dependency>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub content: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub name: String,
    pub content: String,
}
