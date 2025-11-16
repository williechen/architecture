use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug)]
pub enum WebError {
    NotFound(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            WebError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            WebError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl std::error::Error for WebError {}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            WebError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, msg),
            WebError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            WebError::InternalServerError(msg) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        };

        let body = serde_json::json!({
            "status": "error",
            "message": message,
            "data": null
        });

        axum::response::Response::builder()
            .status(status)
            .header(axum::http::header::CONTENT_TYPE, "text/plain")
            .body(axum::body::Body::from(body.to_string()))
            .unwrap()
    }
}
