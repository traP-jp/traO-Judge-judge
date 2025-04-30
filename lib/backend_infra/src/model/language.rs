use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Languages {
    pub languages: Vec<Language>,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub bin_name: String,
    pub compile: String,
    pub run: String,
    #[serde(default)]
    pub libraries: Vec<Library>,
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub version: String,
}
