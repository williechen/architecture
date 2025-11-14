use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            AppError::InternalServerError(msg) => {
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
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::boxed(axum::body::Full::from(body.to_string())))
            .unwrap()
    }
}
