use serde::{Deserialize, Serialize};
use usecase::model::language::LanguageDto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageResponse {
    pub name: String,
}

impl From<LanguageDto> for LanguageResponse {
    fn from(language: LanguageDto) -> Self {
        LanguageResponse {
            name: language.name,
        }
    }
}
