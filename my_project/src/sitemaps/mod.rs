pub mod app_state;

use axum::Router;
use axum::http::StatusCode;
use axum::http::{Request, Uri};
use axum::response::IntoResponse;

use sqlx::SqlitePool;

use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

use crate::api_base::api_errors;
use crate::chapter3;
use crate::sitemaps::app_state::AppState;

pub async fn sitemap(db: SqlitePool) -> Router {
    let app_state = AppState { db: db.clone() };

    let compression = CompressionLayer::new();

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false)
        .max_age(std::time::Duration::from_secs(3600 * 12));

    let timeout = TimeoutLayer::with_status_code(
        StatusCode::REQUEST_TIMEOUT,
        std::time::Duration::from_secs(10),
    );

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
        .merge(chapter3::logic_routes())
        .layer(trace)
        .layer(timeout)
        .layer(cors)
        .layer(compression)
        .fallback(fallback)
        .with_state(app_state.clone())
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    api_errors::ApiError::BadRequest(format!("No route found for {}", uri)).into_response()
}
