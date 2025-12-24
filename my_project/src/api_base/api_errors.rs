use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Field Error: {0}")]
    FieldError(String),
    #[error("JWT Error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("Session Error: {0}")]
    SessionError(#[from] tower_sessions::session::Error),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            ApiError::InternalServerError(msg) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            ApiError::FieldError(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            ApiError::JWTError(err) => (axum::http::StatusCode::BAD_REQUEST, err.to_string()),
            ApiError::SessionError(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            ),
            ApiError::DatabaseError(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            ),
        };

        let body = serde_json::json!({
            "status": "error",
            "message": message,
        });

        axum::response::Response::builder()
            .status(status)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(body.to_string()))
            .unwrap()
    }
}
