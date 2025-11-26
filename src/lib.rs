mod api_base;
pub mod chapter1;
pub mod chapter2;
pub mod configures;
pub mod entities;
pub mod logic;
mod sitemaps;
pub mod tokens;
mod web_base;

use configures::database::DatabaseConfig;

pub async fn run_app() {
    let _logs = configures::get_config().logger.load();
    let db = configures::get_config().database.get_connection().await;
    DatabaseConfig::sync_schema(&db).await;

    let listenert = tokio::net::TcpListener::bind(configures::get_config().server.address())
        .await
        .expect("Failed to bind address");

    tracing::info!(
        "Starting server at {} in {} mode",
        configures::get_config().server.address(),
        configures::get_config().server.app_env()
    );

    tracing::info!("Starting sitemap service...");

    axum::serve(listenert, sitemaps::sitemap(db).await)
        .await
        .unwrap();
}
