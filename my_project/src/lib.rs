pub mod api_base;
pub mod chapter1;
pub mod chapter2;
pub mod chapter3;
pub mod configures;
pub mod entities;
pub mod repositories;
pub mod services;
pub mod sitemaps;

pub async fn run_app() {
    let _logs = configures::get_config().logger.load();
    let db = configures::get_config().database.get_connection().await;
    // Run database migrations
    sqlx::migrate!("./migrations").run(&db).await.unwrap();

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
