use domain::model::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDto {
    pub name: String,
}

impl From<Language> for LanguageDto {
    fn from(val: Language) -> Self {
        LanguageDto { name: val.name }
    }
}
