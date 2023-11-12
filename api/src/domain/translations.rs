use std::error::Error;

pub struct TranslationPage {
    pub id: String,
    pub language: String,
    pub literals: Vec<Literal>
}

pub struct Literal {
    pub id: String,
    pub text: Option<String>,
    pub reviewed: bool,
    pub modified: bool
}

#[async_trait::async_trait]
pub trait TranslationPageRepository {
    async fn get_default_language(&self, page_id: &str) -> Result<Option<TranslationPage>, Box<dyn std::error::Error>>;
    async fn get_language(&self, page_id: &str, language: &str) -> Result<Option<TranslationPage>, Box<dyn std::error::Error>>;
    async fn save(&self, page: &TranslationPage) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_literal(&self, page_id: &str, literal: &str, language: &str) -> Result<Option<String>, Box<dyn Error>>;
}