use std::{error::Error, sync::Arc, net::SocketAddr, collections::HashMap};
use axum::{Router, response::IntoResponse, routing::get, extract::{State, Path}, Json};

use crate::domain::translations::TranslationPageRepository;

pub struct AppContext {
    pub translation_repository: Arc<dyn TranslationPageRepository + Send + Sync>
}

pub async fn start(state: AppContext) -> Result<(), Box<dyn Error>> {
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/:page/:language", get(get_page))
        .route("/:page/:literal/:language", get(get_literal))
        .with_state(shared_state);

    let port = dotenv::var("PORT")
        .unwrap_or("3000".to_owned())
        .parse::<u16>()
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let _ = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_page(
    Path((page, language)): Path<(String, String)>,
    State(context): State<Arc<AppContext>>
) -> impl IntoResponse {
    let result = context.translation_repository.get_language(&page, &language)
        .await;

    if let Ok(Some(page)) = result {
        let mut dictionary = HashMap::<String, String>::new();

        for entry in page.literals {
            if let Some(text) = entry.text {
                dictionary.insert(entry.id, text);
            }
        }

        return Json(dictionary);
    }

    Json(HashMap::new())
}

async fn get_literal(
    Path((page, literal, language)): Path<(String, String, String)>,
    State(context): State<Arc<AppContext>>
) -> impl IntoResponse {
    let result = context.translation_repository
        .get_literal(&page, &literal, &language)
        .await;

    if let Ok(Some(text)) = result {
        return Json(serde_json::Value::String(text));
    }

    Json(serde_json::Value::Null)
}