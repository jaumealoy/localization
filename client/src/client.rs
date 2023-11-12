use std::{error::Error, collections::HashMap};

use async_trait::async_trait;

use crate::{TranslationPage, Localization};

pub struct LocalizationClient {
    client: reqwest::Client,
    base_address: String
}

#[async_trait]
impl Localization for LocalizationClient {
    async fn get_literal(&self, page_id: &str, literal: &str, language: &str) -> Result<Option<String>, Box<dyn Error>> {
        let request_url = format!("{}/{}/{}/{}", self.base_address, page_id, literal, language);
        let response = self.client
            .get(request_url)
            .send()
            .await?
            .json::<Option<String>>()
            .await?;

        Ok(response)
    }

    async fn get_page(&self, page_id: &str, language: &str) -> Result<TranslationPage, Box<dyn Error>> {
        let request_url = format!("{}/{}/{}", self.base_address, page_id, language);
        let response = self.client
            .get(request_url)
            .send()
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        Ok(TranslationPage::from(response))
    }
}

impl LocalizationClient {
    pub fn new(base_address: &str) -> Self {
        let http_client = reqwest::Client::new();

        LocalizationClient {
            base_address: base_address.to_owned(),
            client: http_client
        }
    }
}