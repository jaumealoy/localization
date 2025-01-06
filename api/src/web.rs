use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::{collections::HashMap, error::Error, sync::Arc};

use crate::{
    controllers,
    domain::{
        languages::LanguageRepository, pages::PageRepository,
        translations::TranslationPageRepository,
    },
};

pub struct AppContext {
    pub translation_repository: Arc<dyn TranslationPageRepository + Send + Sync>,
    pub page_repository: Arc<dyn PageRepository + Send + Sync>,
    pub language_repository: Arc<dyn LanguageRepository + Send + Sync>,
}

pub async fn start(state: AppContext) -> Result<(), Box<dyn Error>> {
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/pages", get(controllers::pages::get_pages))
        .route("/pages", post(controllers::pages::create_page))
        .route("/languages", get(controllers::languages::get_languages))
        .route("/languages", post(controllers::languages::update_languages))
        .route(
            "/:page/:language",
            post(controllers::translation_page::save_translation_page),
        )
        .route("/:page/:language", get(get_page))
        .route("/:page/:literal/:language", get(get_literal))
        .with_state(shared_state);

    let port = dotenv::var("PORT")
        .unwrap_or("3000".to_owned())
        .parse::<u16>()
        .unwrap_or(3000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    log::info!("Application is running on port {}", port);

    let _ = axum::serve(listener, app).await?;

    Ok(())
}

async fn get_page(
    Path((page, language)): Path<(String, String)>,
    State(context): State<Arc<AppContext>>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let result = context
        .translation_repository
        .get_language(&page, &language)
        .await;

    let sorted = query.get("sorted").is_some();

    if let Ok(Some(page)) = result {
        if sorted {
            let mut sorted_literals = Vec::with_capacity(page.literals.len());
            for literal in page.literals {
                sorted_literals.push(json!({ "key": literal.id, "text": literal.text }));
            }
            return Json(json!(sorted_literals));
        } else {
            let mut dictionary = HashMap::<String, String>::new();

            for entry in page.literals {
                if let Some(text) = entry.text {
                    dictionary.insert(entry.id, text);
                }
            }

            return Json(json!(dictionary));
        }
    } else if let Err(error) = result {
        log::error!(
            "Error recovering page literals. Page = {}, Language = {}\nError: {:?}",
            &page,
            &language,
            error.to_string()
        );
    }

    Json(if sorted {
        json!(Vec::<serde_json::Value>::new())
    } else {
        json!(HashMap::<String, String>::new())
    })
}

async fn get_literal(
    Path((page, literal, language)): Path<(String, String, String)>,
    State(context): State<Arc<AppContext>>,
) -> impl IntoResponse {
    let result = context
        .translation_repository
        .get_literal(&page, &literal, &language)
        .await;

    if let Ok(Some(text)) = result {
        return Json(serde_json::Value::String(text));
    } else if let Err(error) = result {
        log::error!(
            "Error recovering page literals. Page = {}, Key = {}, Language = {}\nError: {:?}",
            &page,
            &literal,
            &language,
            error.to_string()
        );
    }

    Json(serde_json::Value::Null)
}
