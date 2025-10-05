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
    async fn language_to_id(&self, language: &String) -> anyhow::Result<Option<i32>> {
        Ok(self
            .languages
            .iter()
            .position(|l| l.name == *language)
            .map(|v| v as i32))
    }
    async fn id_to_language(&self, id: i32) -> anyhow::Result<Option<String>> {
        Ok(self.languages.get(id as usize).map(|l| l.name.clone()))
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
        .enumerate()
        .map(|(id, l)| Language {
            id: id as i32,
            name: l.name,
        })
        .collect())
}
