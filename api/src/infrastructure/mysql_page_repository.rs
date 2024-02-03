use std::error::Error;

use async_trait::async_trait;
use mysql_async::{
    params,
    prelude::{Query, WithParams},
    Pool,
};

use crate::domain::pages::{Page, PageRepository};

pub struct MySQLPageRepository(Pool);

#[async_trait]
impl PageRepository for MySQLPageRepository {
    async fn get_pages(&self) -> Result<Vec<Page>, Box<dyn Error>> {
        let connection = self.0.get_conn().await?;
        let pages_result: Vec<String> = r"select id from pages".fetch(connection).await?;

        Ok(pages_result)
    }

    async fn create(&self, page: &Page) -> Result<(), Box<dyn Error>> {
        let connection = self.0.get_conn().await?;
        let _ = r"insert into pages (id) values (:page_id)"
            .with(params! { "page_id" => page })
            .run(connection)
            .await?;

        Ok(())
    }
}

impl MySQLPageRepository {
    pub fn new(pool: Pool) -> Self {
        Self(pool)
    }
}
