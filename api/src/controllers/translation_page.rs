use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{domain::translations::{Literal, TranslationPage}, web::AppContext};

#[derive(Deserialize)]
pub struct FormLiteral {
    pub key: String,
    pub value: String
}

pub async fn save_translation_page(
    State(app_state): State<Arc<AppContext>>,
    Path((page_id, language)): Path<(String, String)>,
    Json(body): Json<Vec<FormLiteral>>
) -> impl IntoResponse {

    let page = TranslationPage {
        id: page_id,
        language,
        literals: body.iter()
            .map(|literal| Literal {
                id: literal.key.to_owned(),
                text: Some(literal.value.to_owned()),
                reviewed: false,
                modified: true
            })
            .collect::<Vec<_>>()
    };

    match app_state.translation_repository.save(&page).await {
        Ok(()) => StatusCode::OK,
        Err(error) => {
            log::error!("Error while saving literals: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}