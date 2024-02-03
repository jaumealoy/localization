use std::error::Error;

use async_trait::async_trait;
use serde::Serialize;

pub struct Language {
    pub code: String,
    pub default_language: bool,
}

#[async_trait]
pub trait LanguageRepository {
    async fn get_languages(&self) -> Result<Vec<Language>, Box<dyn Error>>;
    async fn save_languages(&self, languages: &Vec<Language>) -> Result<(), Box<dyn Error>>;
}