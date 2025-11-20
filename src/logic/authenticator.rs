use std::pin::Pin;

use axum::{body::Body, http::Request, http::Response};
use serde::{Deserialize, Serialize};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

use crate::tokens::jwt::{JWT, JwtConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth;

pub struct JwtToken {
    pub token: JWT,
}

impl JwtToken {
    pub fn new(config: JwtConfig) -> Self {
        let token = JWT::new(config);
        JwtToken { token }
    }
}

impl AsyncAuthorizeRequest<Body> for JwtToken {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        let jwt = self.token.clone();

        Box::pin(async move {
            let auth_header = request
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            if let Some(auth_value) = auth_header {
                if auth_value.starts_with("Bearer ") {
                    let token_str = &auth_value[7..];
                    let token_valid = jwt.decode::<Auth>(token_str);
                    if token_valid.is_ok() {
                        request.extensions_mut().insert(token_valid.unwrap());
                        return Ok(request);
                    }
                }
            }

            let response = Response::builder()
                .status(401)
                .body(Body::from("Unauthorized"))
                .unwrap();
            Err(response)
        })
    }
}

pub fn authenticator_layer(config: JwtConfig) -> AsyncRequireAuthorizationLayer<JwtToken> {
    AsyncRequireAuthorizationLayer::new(JwtToken::new(config))
}
