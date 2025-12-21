use crate::{
    api_base::api_errors::ApiError,
    sitemaps::app_state::AppState,
    tokens::{auth_user, jwt::JWT},
};
use axum::{Form, Json, Router, routing::post};
use serde::Deserialize;

pub fn logic_routes() -> Router<AppState> {
    Router::new().route("/login/auth", post(login_auth))
}

#[derive(Deserialize)]
pub struct AuthVo {
    username: String,
    password: String,
}

pub async fn login_auth(Form(auth): Form<AuthVo>) -> Result<Json<String>, ApiError> {
    let config = crate::tokens::jwt::JwtConfig::default();
    let jwt = JWT::new(config);

    let user = auth_user::User::new("some_id".to_string());

    let token = jwt.encode(user)?;

    Ok(Json(format!(
        "Authenticated user: {}, password: {}, token: {}",
        auth.username, auth.password, token
    )))
}
