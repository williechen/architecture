use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::http::{HeaderMap, header};
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
            return Err(ApiError::Unauthorized(
                "Expected `Content-Type: application/json`".to_string(),
            ));
        }

        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| ApiError::Unauthorized("Failed to buffer request body".to_string()))?;

        let aes_str = String::from_utf8(bytes.to_vec()).map_err(|_| {
            ApiError::Unauthorized("Failed to parse request body as UTF-8".to_string())
        })?;

        // 2. Base64 decode
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(aes_str.trim())
            .map_err(|_| ApiError::Unauthorized("Failed to decode base64".to_string()))?;

        // 3. JSON deserialize
        let payload: T = serde_json::from_slice(&decoded)
            .map_err(|_| ApiError::Unauthorized("Failed to parse JSON".to_string()))?;

        Ok(SecurityJson(payload))
    }
}

fn json_content_type(headers: &HeaderMap) -> bool {
    let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
        return false;
    };

    let Ok(content_type) = content_type.to_str() else {
        return false;
    };

    content_type.starts_with("application/json")
}
