pub mod chapter1;
pub mod chapter2;
pub mod configures;
pub mod sitemaps;

use crate::configures::application;

pub async fn run_app() {
    let _logs = application::get_config().logger.load();
    let db = application::get_config().database.get_connection().await;

    tracing::info!(
        "Starting server at {} in {} mode",
        application::get_config().server.address(),
        application::get_config().server.app_env()
    );

    tracing::info!("Starting sitemap service...");

    let listenert = tokio::net::TcpListener::bind(application::get_config().server.address())
        .await
        .expect("Failed to bind address");

    axum::serve(listenert, sitemap::sitemap(db)).await.unwrap();
}
