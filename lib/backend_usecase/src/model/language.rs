use domain::model::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDto {
    pub id: String,
    pub name: String,
}

impl From<Language> for LanguageDto {
    fn from(val: Language) -> Self {
        LanguageDto {
            id: val.id,
            name: val.name,
        }
    }
}
