use log::error;
use mysql_async::{Pool, prelude::{Query, WithParams}, Row, Conn, params};

use crate::domain::translations::{TranslationPageRepository, TranslationPage, Literal};

pub struct MySQLTranslationPageRepository(Pool);

#[async_trait::async_trait]
impl TranslationPageRepository for MySQLTranslationPageRepository {
    async fn save(&self, page: &TranslationPage) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get_default_language(&self, page_id: &str) -> Result<Option<TranslationPage>, Box<dyn std::error::Error>> {
        let mut connection = self.0
            .get_conn()
            .await?;

        let result = r"select code from languages where isDefaultLanguage = 1"
            .first::<Row, &mut Conn>(&mut connection)
            .await?;

        if let Some(row) = result {
            let language = row.get::<String, &str>("code").unwrap();
            self.get_language(page_id, &language).await
        } else {
            error!("There is no default language set");
            Ok(None)
        }
    }

    async fn get_language(&self, page_id: &str, language: &str) -> Result<Option<TranslationPage>, Box<dyn std::error::Error>> {
        let mut connection = self.0
            .get_conn()
            .await?;

        // check that the page existis
        let page_result = r"select count(*) from pages where id = :page_id"
            .with(params! { "page_id" => page_id })
            .first::<Row, &mut Conn>(&mut connection)
            .await?;

        if page_result.is_none() || page_result.unwrap().get::<u32, usize>(0).unwrap() == 0 {
            return Ok(None);
        }

        let rows = r"select 
            m.`key`, t.`text`, IF(t.reviewed is null, false, t.reviewed) as reviewed
            from languages
                join literals m on m.pageId = :page_id and m.language = languages.code
                left join literals t on t.pageId = m.pageId and m.key = t.key and t.language = :language
            where languages.isDefaultLanguage = 1"
            .with(params! {
                "page_id" => page_id,
                "language" => language
            })
            .fetch::<Row, &mut Conn>(&mut connection)
            .await?;

        let page = TranslationPage {
            id: page_id.to_owned(),
            language: language.to_owned(),
            literals: rows.iter().map(|row| Literal {
                id: row.get("key").unwrap(),
                text: row.get("text").unwrap(),
                reviewed: row.get("reviewed").unwrap(),
                modified: false
            }).collect::<Vec<_>>()
        };

        Ok(Some(page))
    }

    async fn get_literal(&self, page_id: &str, literal: &str, language: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut connection = self.0
            .get_conn()
            .await?;

        let result = r"select `text` from literals 
            where pageId = :page_id and `key` = :key and language = :language limit 1"
            .with(params! {
                "page_id" => page_id,
                "key" => literal,
                "language" => language
            })
            .first::<Row, &mut Conn>(&mut connection)
            .await?;

        if let Some(row) = result {
            Ok(row.get::<String, &str>("text"))
        } else {
            Ok(None)
        }
    }
}

impl MySQLTranslationPageRepository {
    pub fn new(pool: Pool) -> Self {
        MySQLTranslationPageRepository(pool)
    }
}