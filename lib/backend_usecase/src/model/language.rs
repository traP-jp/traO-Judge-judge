use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDto {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagesDto {
    pub languages: Vec<LanguageDto>,
}
