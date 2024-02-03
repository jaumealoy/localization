use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{domain::languages::Language, web::AppContext};

pub async fn get_languages(
    State(app_state): State<Arc<AppContext>>
) -> impl IntoResponse {
    let languages_result = app_state.language_repository.get_languages().await;

    match languages_result {
        Ok(languages) => {
            let mapped_languages = languages.iter()
                .map(|language| json!({ "code": language.code, "default": language.default_language }))
                .collect::<Vec<_>>();
            (StatusCode::OK, Json(json!(mapped_languages)))
        },
        Err(error) => {
            log::error!("Error while fetching available languages: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

#[derive(Deserialize)]
pub struct FormLanguage {
    pub code: String,
    pub default: bool,
}

pub async fn update_languages(
    State(app_state): State<Arc<AppContext>>,
    Json(body): Json<Vec<FormLanguage>>
) -> impl IntoResponse {
    if !body.iter().any(|x| x.default) {
        return (StatusCode::BAD_REQUEST, Json(json!("Set a default language")));
    }

    let mapped_langues = body.iter()
        .map(|language| Language {
            code: language.code.to_owned(),
            default_language: language.default
        })
        .collect::<Vec<_>>();

    let result = app_state.language_repository
        .save_languages(&mapped_langues)
        .await;
        
    match result {
        Ok(_) => (StatusCode::OK, Json(Value::Bool(true))),
        Err(error) => {
            log::error!("Unable to save languages: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Bool(false)))
        }
    }
}