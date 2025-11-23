pub mod app_state;

use std::sync::Arc;
use tokio::sync::RwLock;

use axum::Router;
use axum::http::{Request, Uri};
use axum::response::IntoResponse;
use rbatis::rbatis::RBatis;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

use crate::logic;
use crate::sitemaps::app_state::AppState;
use crate::web_base::web_errors;

pub async fn sitemap(db: RBatis) -> Router {
    let app_state = AppState {
        db: db.clone(),
        codemap: Arc::new(RwLock::new(app_state::load_codemap(&db).await)),
        config: Arc::new(RwLock::new(app_state::load_config(&db).await)),
    };
    app_state::load(app_state.clone()).await;

    let compression = CompressionLayer::new();

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false)
        .max_age(std::time::Duration::from_secs(3600 * 12));

    let timeout = TimeoutLayer::new(std::time::Duration::from_secs(10));

    let trace = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<axum::body::Body>| {
            let method = request.method();
            let path = request.uri().path();
            let headers = request.headers();

            tracing::info_span!(
                "sitemap",
                headers = ?headers,
                method = %method,
                uri = %path,
            )
        })
        .on_failure(())
        .on_request(())
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));

    Router::new()
        .merge(logic::common::common_routes().await)
        .merge(logic::logic_routes().await)
        .nest_service("/plugins", ServeDir::new("static/"))
        .layer(trace)
        .layer(timeout)
        .layer(cors)
        .layer(compression)
        .fallback(fallback)
        .with_state(app_state.clone())
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    web_errors::WebError::NotFound(format!("No route for {}", uri)).into_response()
}
