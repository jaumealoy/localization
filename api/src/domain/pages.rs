use async_trait::async_trait;
use std::error::Error;

pub type Page = String;

#[async_trait]
pub trait PageRepository {
    async fn get_pages(&self) -> Result<Vec<Page>, Box<dyn Error>>;
    async fn create(&self, page: &Page) -> Result<(), Box<dyn Error>>;
}
