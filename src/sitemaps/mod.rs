use axum::{Router, routing::get};
pub fn sitemap(db: Rbatis) -> Router {
    Router::new()
        .route("/sitemap.xml", get(|| async { "Sitemap XML content" }))
        .fallback(fallback)
        .with_state(db)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
