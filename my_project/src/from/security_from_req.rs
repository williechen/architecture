use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::http::HeaderMap;
use base64::Engine;
use serde::de::DeserializeOwned;

use crate::api_base::api_errors::ApiError;

#[derive(Debug)]
pub struct SecurityJson<T>(pub T);

impl<T, S> FromRequest<S> for SecurityJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !json_content_type(req.headers()) {
            return Err(ApiError::BadRequest(
                "Expected `Content-Type: application/json`".to_string(),
            ));
        }

        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| ApiError::FieldError("Failed to buffer request body".to_string()))?;

        let aes_str = String::from_utf8(bytes.to_vec()).map_err(|_| {
            ApiError::FieldError("Failed to parse request body as UTF-8".to_string())
        })?;

        // 2. Base64 decode
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(aes_str.trim())
            .map_err(|_| ApiError::FieldError("Failed to decode base64".to_string()))?;

        // 3. JSON deserialize
        let payload: T = serde_json::from_slice(&decoded)
            .map_err(|_| ApiError::FieldError("Failed to parse JSON".to_string()))?;
        Ok(SecurityJson(payload))
    }
}

fn json_content_type(headers: &HeaderMap) -> bool {
    headers
        .get("Accept")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("application/json"))
        .unwrap_or(false)
}
