use std::pin::Pin;

use axum::response::IntoResponse;
use axum::{
    body::Body,
    http::StatusCode,
    http::{Request, Response},
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

use crate::tokens::jwt::{JWT, JwtConfig};

fn should_skip(path: &str, skips: &[String]) -> bool {
    skips.iter().any(|p| {
        if p.ends_with('*') {
            path.starts_with(p.trim_end_matches('*'))
        } else {
            path == p
        }
    })
}

fn is_browser(req: &Request<Body>) -> bool {
    req.headers()
        .get("Accept")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/html"))
        .unwrap_or(false)
}

async fn get_user_acl(db: &sqlx::SqlitePool, user_id: String) -> Vec<String> {
    let sql = format!(
        "SELECT acl_code
             FROM uam_acl ua
             JOIN uam_role_acl ura 
               ON ua.acl_code = ura.acl_code
             JOIN uam_user_role uur
               ON ura.role_code = uur.role_code
            WHERE user_id = '{}'
        ",
        user_id
    );

    let acl_codes: Vec<String> = sqlx::query_as::<_, (String,)>(&sql)
        .fetch_all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| row.0)
        .collect();

    acl_codes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub user_id: String,
}

#[derive(Clone)]
pub struct JwtToken {
    pub token: JWT,
    pub skip_paths: Vec<String>,
}

impl JwtToken {
    pub fn new(config: JwtConfig, skip_paths: Vec<String>) -> Self {
        let token = JWT::new(config);
        JwtToken { token, skip_paths }
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
        let path = request.uri().path().to_string();
        let jwt = self.token.clone();

        // 🚪 排除路徑 → 直接放行
        if should_skip(&path, &self.skip_paths) {
            return Box::pin(async move { Ok(request) });
        }

        Box::pin(async move {
            let auth_header = request
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            if let Some(auth_value) = auth_header {
                if auth_value.starts_with("Bearer ") {
                    let token_str = &auth_value[7..];
                    let token_valid = jwt.decode::<Authorization>(token_str);
                    if token_valid.is_ok() {
                        request.extensions_mut().insert(token_valid.unwrap());
                        return Ok(request);
                    }
                }
            } else {
                // Check for token in cookies
                if let Some(cookie_header) = request.headers().get("Cookie") {
                    if let Ok(cookie_str) = cookie_header.to_str() {
                        for cookie in cookie_str.split(';') {
                            let cookie = cookie.trim();
                            if cookie.starts_with("auth_token=") {
                                let token_str = &cookie["auth_token=".len()..];
                                let token_valid = jwt.decode::<Authorization>(token_str);
                                if token_valid.is_ok() {
                                    request.extensions_mut().insert(token_valid.unwrap());
                                    return Ok(request);
                                }
                            }
                        }
                    }
                }
            }

            if is_browser(&request) {
                Err(Redirect::to("/login").into_response())
            } else {
                Err(StatusCode::UNAUTHORIZED.into_response())
            }
        })
    }
}

pub fn authenticator_layer(
    config: JwtConfig,
    skip_paths: Vec<String>,
) -> AsyncRequireAuthorizationLayer<JwtToken> {
    AsyncRequireAuthorizationLayer::new(JwtToken::new(config, skip_paths))
}
