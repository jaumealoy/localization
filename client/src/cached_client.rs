use std::{time::Duration, error::Error, collections::HashMap, sync::{RwLock, Arc}, alloc::System, hash::Hash};
use async_trait::async_trait;
use crate::{LocalizationClient, Localization, TranslationPage};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CachedLocalizationClient {
    inner: LocalizationClient,
    cache_interval: Duration,
    literals_cache: Arc<RwLock<HashMap<String, (u64, String)>>>,
    pages_cache: Arc<RwLock<HashMap<String, (u64, TranslationPage)>>>
}

impl CachedLocalizationClient {
    pub fn new(cache_interval: Duration, base_address: &str) -> Self {
        CachedLocalizationClient { 
            inner: LocalizationClient::new(base_address), 
            cache_interval: cache_interval,
            literals_cache: Arc::new(RwLock::new(HashMap::new())),
            pages_cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

#[async_trait]
impl Localization for CachedLocalizationClient {
    async fn get_literal(&self, page_id: &str, literal: &str, language: &str) -> Result<Option<String>, Box<dyn Error>> {
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH)?;
        
        let literal_key = format!("{}#{}#{}", page_id, literal, language);
        let mut expired_cache = false;

        {
            let read_lock = self.literals_cache.read().unwrap();
            if let Some(entry) = read_lock.get(&literal_key) {
                let cache_time = UNIX_EPOCH + Duration::from_secs(entry.0);
                expired_cache = start.duration_since(cache_time).unwrap() > self.cache_interval;
            } else {
                expired_cache = true;
            }
        }

        if expired_cache {
            // fetch the literal
            let value = self.inner
                .get_literal(page_id, literal, language)
                .await?;

            if value.is_none() {
                return Ok(None);
            }

            // store the value in cache
            let mut lock = self.literals_cache
                .write()
                .unwrap();

            if lock.contains_key(&literal_key) {
                let entry = lock.get_mut(&literal_key).unwrap();
                *entry = (now.as_secs(), value.clone().unwrap());
            } else {
                lock.insert(
                    literal_key.to_owned(), 
                    (now.as_secs(), value.clone().unwrap())
                );
            }

            return Ok(value);
        } else {
            // this literal is cached
            let read_lock = self.literals_cache.read().unwrap();
            let result = read_lock.get(&literal_key).unwrap();
            return Ok(Some(result.1.to_owned()));
        }
    }

    async fn get_page(&self, page_id: &str, language: &str) -> Result<TranslationPage, Box<dyn Error>> {
        let start = SystemTime::now();
        let now = start.duration_since(UNIX_EPOCH)?;
        
        let page_key = format!("{}#{}#{}", page_id, page_id, language);
        let mut expired_cache = false;

        {
            let read_lock = self.pages_cache.read().unwrap();
            if let Some(entry) = read_lock.get(&page_key) {
                let cache_time = UNIX_EPOCH + Duration::from_secs(entry.0);
                expired_cache = start.duration_since(cache_time).unwrap() > self.cache_interval;
            } else {
                expired_cache = true;
            }
        }

        if expired_cache {
            // fetch the literal
            let value = self.inner
                .get_page(page_id, language)
                .await?;

            // store the value in cache
            let mut lock = self.pages_cache
                .write()
                .unwrap();

            if lock.contains_key(&page_key) {
                let entry = lock.get_mut(&page_key).unwrap();
                *entry = (now.as_secs(), value.clone());
            } else {
                lock.insert(
                    page_key.to_owned(), 
                    (now.as_secs(), value.clone())
                );
            }

            return Ok(value);
        } else {
            // this literal is cached
            let read_lock = self.pages_cache.read().unwrap();
            let result = read_lock.get(&page_key).unwrap();
            return Ok(result.1.clone());
        }
    }
}