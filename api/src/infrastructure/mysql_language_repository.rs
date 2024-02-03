use std::{any::type_name, error::Error};

use async_trait::async_trait;
use mysql_async::{params, prelude::{FromValue, Query, WithParams}, Pool, Row, TxOpts, Value};

use crate::domain::languages::{Language, LanguageRepository};

pub struct MySQLLanguageRepository(Pool);

#[async_trait]
impl LanguageRepository for MySQLLanguageRepository {
    async fn get_languages(&self) -> Result<Vec<Language>, Box<dyn Error>> {
        let connection = self.0.get_conn().await?;

        let results: Vec<Row> = r"select code, isDefaultLanguage from languages"
            .fetch(connection)
            .await?;

        let languages = results.iter()
            .map(|row| {
                let boolean_value = row.get::<Vec<u8>, &str>("isDefaultLanguage").unwrap();
                Language {
                    code: row.get::<String, &str>("code").unwrap(),
                    default_language: boolean_value.len() == 1 && boolean_value[0] > 0,
                }
            })
            .collect::<Vec<_>>();

        Ok(languages)
    }

    async fn save_languages(&self, languages: &Vec<Language>) -> Result<(), Box<dyn Error>> {
        let existing_languages = self.get_languages().await?;

        let mut connection = self.0.get_conn().await?;

        let options = TxOpts::default();
        let mut tx = connection.start_transaction(options).await?;

        for language in languages {
            // create only new languages
            if !existing_languages.iter().any(|other| other.code == language.code) {
                r"insert into languages values (:code, 0)"
                    .with(params! { "code" => &language.code })
                    .run(&mut tx)
                    .await?;
            }
        }

        // set default language
        r"update languages set isDefaultLanguage = 0".run(&mut tx).await?;
        r"update languages set isDefaultLanguage = 1 where code = :code"
            .with(params! {
                "code" => &languages.iter().find(|x| x.default_language).unwrap().code,
            })
            .run(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}


impl MySQLLanguageRepository {
    pub fn new(pool: Pool) -> Self {
        Self(pool)
    }
}

