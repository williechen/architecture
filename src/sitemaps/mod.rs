use axum::http::{StatusCode, Uri};
use axum::{Router, routing::get};
use rbatis::rbatis::RBatis;

pub fn sitemap(db: RBatis) -> Router {
    Router::new()
        .route("/sitemap.xml", get(|| async { "Sitemap XML content" }))
        .fallback(fallback)
        .with_state(db)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
