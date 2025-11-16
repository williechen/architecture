pub mod api_errors;
pub mod api_response;
pub mod web_errors;
pub mod web_response;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::http::{Request, StatusCode, Uri};
use axum::{Router, routing::get};
use rbatis::rbatis::RBatis;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: RBatis,
    pub codemap: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    pub config: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

impl AppState {
    pub fn new(db: RBatis) -> Self {
        Self {
            db,
            codemap: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_codemap(&self, db: RBatis) -> Arc<RwLock<HashMap<String, HashMap<String, String>>>> {
        self.codemap.clone()
    }
    pub fn get_config(&self, db: RBatis) -> Arc<RwLock<HashMap<String, HashMap<String, String>>>> {
        self.config.clone()
    }
}

pub fn sitemap(db: RBatis) -> Router {
    let compression = CompressionLayer::new();

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false)
        .max_age(Duration::from_secs(3600 * 12));

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
        .route("/sitemap.xml", get(|| async { "Sitemap XML content" }))
        .layer(trace)
        .layer(timeout)
        .layer(cors)
        .layer(compression)
        .fallback(fallback)
        .with_state(AppState::new(db))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
