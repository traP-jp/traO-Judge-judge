use domain::model::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageRow {
    pub name: String,
}

impl From<LanguageRow> for Language {
    fn from(val: LanguageRow) -> Self {
        Language { name: val.name }
    }
}
