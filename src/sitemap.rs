use axum::{Router, routing::get};
pub fn sitemap() -> Router {
    Router::new().route("/sitemap.xml", get(|| async { "Sitemap XML content" }))
}
