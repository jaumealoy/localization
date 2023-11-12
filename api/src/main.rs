use std::sync::Arc;
use infrastructure::mysql_translation_page_repository::MySQLTranslationPageRepository;
use log::error;
use mysql_async::{Pool, OptsBuilder};
use web::AppContext;

mod domain;
mod infrastructure;
mod web;

#[tokio::main]
async fn main() {
    if let Err(_) = dotenv::dotenv() {
        error!("Error loading envrionment variables");
        std::process::exit(-1);
    }

    env_logger::init();

    // connecting to database
    let connection_options = OptsBuilder::default()
        .ip_or_hostname(dotenv::var("DB_HOST").unwrap())
        .user(
            if let Ok(value) = dotenv::var("DB_USER") {
                Some(value)
            } else {
                None
            }
        )
        .pass(
            if let Ok(value) = dotenv::var("DB_PASS") {
                Some(value)
            } else {
                None
            }
        )
        .db_name(
            if let Ok(value) = dotenv::var("DB_NAME") {
                Some(value)
            } else {
                None
            }
        );
    let pool = Pool::new(connection_options);

    // initialize all dependencies
    let translation_repository = MySQLTranslationPageRepository::new(pool);

    let state = AppContext {
        translation_repository: Arc::new(translation_repository)
    };

    // start web server
    let _ = web::start(state).await;
}
