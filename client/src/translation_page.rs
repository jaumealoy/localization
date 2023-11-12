use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TranslationPage(HashMap<String, String>);

impl TranslationPage {
    pub fn get_translation(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn get_translation_or_default<'a>(&'a self, key: &str, default: &'a String) -> &str {
        self.0.get(key)
            .unwrap_or(default)
    }
}

impl From<HashMap<String, String>> for TranslationPage {
    fn from(value: HashMap<String, String>) -> Self {
        TranslationPage(value)
    }
}
