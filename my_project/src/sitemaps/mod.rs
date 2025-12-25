pub mod app_state;
mod authenticator;
mod csp_layer;

use std::sync::Arc;

use axum::Router;
use axum::http::StatusCode;
use axum::http::{Request, Uri};
use axum::response::IntoResponse;

use sqlx::SqlitePool;

use tokio::sync::RwLock;

use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_sessions::cookie::time;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_base::api_doc::ApiDoc;
use crate::sitemaps::app_state::AppState;
use crate::sitemaps::authenticator::authenticator_layer;
use crate::web_base::web_errors;
use crate::{chapter3, logic};

pub async fn sitemap(db: SqlitePool) -> Router {
    let app_state = AppState {
        db: db.clone(),
        codemap: Arc::new(RwLock::new(app_state::load_codemap(&db).await)),
        config: Arc::new(RwLock::new(app_state::load_config(&db).await)),
    };
    app_state::load(&app_state).await;

    let compression = CompressionLayer::new();

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_credentials(false)
        .max_age(std::time::Duration::from_secs(3600 * 12));

    let csp_layer = csp_layer::ContentSecurityPolicyLayer::new()
        .set_default_src("'self'")
        .set_script_src("'self'")
        .set_style_src("'self'");

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

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // 開發用，prod 設 true + https
        .with_expiry(Expiry::OnInactivity(time::Duration::days(7)));

    let config = crate::auth::auth_jwt::JwtConfig::default();
    let skip_paths = vec![
        String::from("/login/*"),
        String::from("/health"),
        String::from("/assets/*"),
        String::from("/swagger/*"),
        String::from("/api-doc/*"),
        String::from("/allocate"),
    ];

    Router::new()
        .merge(logic::uam::logic_routes())
        .merge(logic::common::common_routes())
        .merge(logic::logic_routes())
        .merge(chapter3::logic_routes())
        .merge(
            SwaggerUi::new("/swagger") // 用於 UI 的 endpoint
                .url("/api-doc/openapi.json", ApiDoc::openapi()), // 提供 openapi.json 的路徑與內容
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(trace)
        .layer(timeout)
        .layer(authenticator_layer(config, skip_paths, app_state.clone()))
        .layer(session_layer)
        .layer(csp_layer)
        .layer(cors)
        .layer(compression)
        .fallback(fallback)
        .with_state(app_state.clone())
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    web_errors::WebError::NotFound(format!("No route for {}", uri)).into_response()
}
