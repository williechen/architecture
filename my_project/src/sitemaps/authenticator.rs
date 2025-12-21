use std::pin::Pin;

use axum::response::IntoResponse;
use axum::{
    body::Body,
    http::StatusCode,
    http::{Request, Response},
    response::Redirect,
};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};
use tower_sessions::Session;

use crate::tokens::auth_perm::{self, Permission};
use crate::tokens::jwt::{JWT, JwtConfig};

fn should_skip(path: &str, skips: &[String]) -> bool {
    skips.iter().any(|p| {
        if p.ends_with('*') {
            path.starts_with(p.trim_end_matches("/*"))
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

fn is_permission(
    path: &str,
    permission: &Permission,
    request: Request<Body>,
) -> Result<Request<Body>, Response<Body>> {
    let newpermission = permission.build();

    if newpermission.has_api_module_perms(path)
        || newpermission.has_web_module_perms(path)
        || newpermission.has_api_perm(path)
        || newpermission.has_web_perm(path)
    {
        Ok(request)
    } else {
        if is_browser(&request) {
            Err(Redirect::to("/404").into_response())
        } else {
            Err(StatusCode::FORBIDDEN.into_response())
        }
    }
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
            let session = request.extensions().get::<Session>().cloned();

            if let Some(session) = session {
                // 例如：login user id
                let token_str: Option<String> = session.get("token").await.unwrap();
                if let Some(token_str) = token_str {
                    let token_valid = jwt.decode::<auth_perm::Permission>(&token_str);
                    if token_valid.is_ok() {
                        let permission = token_valid.unwrap();
                        request.extensions_mut().insert(permission.clone());
                        return is_permission(&path, &permission, request);
                    }
                }
            }

            let auth_header = request
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            if let Some(auth_value) = auth_header {
                if auth_value.starts_with("Bearer ") {
                    let token_str = &auth_value[7..];
                    let token_valid = jwt.decode::<auth_perm::Permission>(token_str);
                    if token_valid.is_ok() {
                        let permission = token_valid.unwrap();
                        request.extensions_mut().insert(permission.clone());
                        return is_permission(&path, &permission, request);
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
