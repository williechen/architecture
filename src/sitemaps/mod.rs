mod api_errors;
mod app_state;
mod web_errors;

use axum::Json;
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::RwLock;

use axum::http::{Request, Uri};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use rbatis::rbatis::RBatis;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

use crate::sitemaps::app_state::AppState;

pub async fn sitemap(db: RBatis) -> Router {
    let mut app_state = AppState {
        db: db.clone(),
        codemap: Arc::new(RwLock::new(app_state::load_codemap(&db).await)),
        config: Arc::new(RwLock::new(app_state::load_config(&db).await)),
    };
    app_state::load(&mut app_state).await;

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
        .route("/cache", get(get_cache))
        .layer(trace)
        .layer(timeout)
        .layer(cors)
        .layer(compression)
        .fallback(fallback)
        .with_state(app_state)
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    web_errors::WebError::NotFound(format!("No route for {}", uri)).into_response()
}

async fn get_cache(State(state): State<AppState>) -> Json<String> {
    let data = state.codemap.read().await;
    let json_str = serde_json::to_string(&*data).unwrap_or_else(|_| "{}".to_string());
    Json(json_str)
}
