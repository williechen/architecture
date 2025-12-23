use crate::{
    api_base::api_errors::ApiError,
    sitemaps::app_state::AppState,
    tokens::{auth_user, jwt::JWT},
};
use axum::{Form, Json, Router, extract::State, routing::post};
use serde::Deserialize;
use tower_sessions::Session;

pub fn logic_routes() -> Router<AppState> {
    Router::new().route("/login/auth", post(login_auth))
}

#[derive(Deserialize)]
pub struct AuthVo {
    username: String,
    password: String,
}

pub async fn login_auth(
    State(app_state): State<AppState>,
    session: Session,
    Form(auth): Form<AuthVo>,
) -> Result<Json<String>, ApiError> {
    let config = crate::tokens::jwt::JwtConfig::default();
    let jwt = JWT::new(config);

    let user = auth_user::User::new("some_id".to_string());

    let token = jwt.encode(user)?;

    session.insert("token", token.clone()).await?;

    Ok(Json(format!(
        "Authenticated user: {}, password: {}, token: {}",
        auth.username, auth.password, token
    )))
}
