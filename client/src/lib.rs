use std::{error::Error, collections::HashMap, future::Future};
use async_trait::async_trait;

mod client;
pub use client::LocalizationClient;

mod cached_client;
pub use cached_client::CachedLocalizationClient;

mod translation_page;
pub use translation_page::TranslationPage;

#[async_trait]
pub trait Localization {
    async fn get_literal(&self, page_id: &str, literal: &str, language: &str) ->Result<Option<String>, Box<dyn Error>>;
    async fn get_page(&self, page_id: &str, language: &str) -> Result<TranslationPage, Box<dyn Error>>;
}