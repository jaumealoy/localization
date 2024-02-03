use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::web::AppContext;

pub async fn get_pages(State(app_state): State<Arc<AppContext>>) -> impl IntoResponse {
    let pages_result = app_state.page_repository.get_pages().await;

    match pages_result {
        Ok(pages) => (StatusCode::OK, Json(json!(pages))),
        Err(error) => {
            log::error!("Error fetching available pages: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Value::Null))
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePageBody {
    pub id: String,
}

pub async fn create_page(
    State(app_state): State<Arc<AppContext>>,
    Json(body): Json<CreatePageBody>,
) -> impl IntoResponse {
    let result = app_state.page_repository.create(&body.id).await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(error) => {
            log::error!("Error creating new page: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
