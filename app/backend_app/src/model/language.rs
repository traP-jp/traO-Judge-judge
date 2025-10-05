use serde::{Deserialize, Serialize};
use usecase::model::language::LanguageDto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageResponse {
    pub id: String,
    pub name: String,
}

impl From<LanguageDto> for LanguageResponse {
    fn from(language: LanguageDto) -> Self {
        LanguageResponse {
            id: language.id.to_string(),
            name: language.name,
        }
    }
}
