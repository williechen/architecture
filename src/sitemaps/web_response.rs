use axum::response::IntoResponse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WebResponse<T> {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> IntoResponse for WebResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| {
            r#"{"status":"error","message":"Failed to serialize response","data":null}"#.to_string()
        });
        axum::response::Response::builder()
            .header(axum::http::header::CONTENT_TYPE, "text/plain")
            .body(axum::body::Body::from(body))
            .unwrap()
    }
}
