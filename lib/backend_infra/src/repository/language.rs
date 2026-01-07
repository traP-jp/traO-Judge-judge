use axum::async_trait;
use domain::{model::language::Language, repository::language::LanguageRepository};

use crate::model::language::LanguagesRow;

use std::{env, fs};

#[derive(Clone)]
pub struct LanguageRepositoryImpl {
    pub languages: Vec<Language>,
}

impl LanguageRepositoryImpl {
    pub fn new() -> Self {
        LanguageRepositoryImpl {
            languages: load_languages().unwrap_or(Vec::new()),
        }
    }
}

#[async_trait]
impl LanguageRepository for LanguageRepositoryImpl {
    async fn get_languages(&self) -> anyhow::Result<Vec<Language>> {
        Ok(self.languages.clone())
    }
    async fn language_to_id(&self, language: String) -> anyhow::Result<Option<String>> {
        // 存在するか否かを調べる
        for lang in &self.languages {
            if lang.name == language {
                return Ok(Some(lang.id.clone()));
            }
        }
        Ok(None)
    }

    async fn id_to_language(&self, id: String) -> anyhow::Result<Option<String>> {
        for lang in &self.languages {
            if lang.id == id {
                return Ok(Some(lang.name.clone()));
            }
        }
        Ok(None)
    }
}

fn load_languages() -> anyhow::Result<Vec<Language>> {
    let var_name = "LANGUAGES_PATH";
    let path = env::var(var_name)?;
    let s = fs::read_to_string(path)?;
    let languages: LanguagesRow = serde_json::from_str(&s)?;
    Ok(languages
        .languages
        .into_iter()
        .map(|l| Language {
            id: l.name.clone(),
            name: l.name,
        })
        .collect())
}
