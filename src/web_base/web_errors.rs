use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum WebError {
    #[error("Not Found of {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Render Error: {0}")]
    Render(#[from] askama::Error),
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            status: u16,
            status_code: String,
            message: String,
        }

        let (status, message) = match self {
            WebError::NotFound(msg) => (axum::http::StatusCode::NOT_FOUND, msg),
            WebError::Unauthorized(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            WebError::InternalServerError(msg) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            WebError::Render(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            ),
        };

        let tmpl = Tmpl {
            status: status.as_u16(),
            status_code: status.to_string(),
            message,
        };

        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
