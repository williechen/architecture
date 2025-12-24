use crate::{
    api_base::api_errors::ApiError,
    auth::{auth_jwt::JWT, auth_user},
    entities::uam_user::UamUser,
    repositories::{create, read_one},
    sitemaps::app_state::AppState,
};
use axum::{Form, Json, Router, extract::State, routing::post};
use chrono::Utc;
use serde::Deserialize;
use tower_sessions::Session;

pub fn logic_routes() -> Router<AppState> {
    Router::new()
        .route("/login/auth", post(login_auth))
        .route("/login/create", post(create_user))
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
    let config = crate::auth::auth_jwt::JwtConfig::default();
    let jwt = JWT::new(config);

    let uam_user = read_one::<UamUser>(
        &app_state.db,
        &UamUser::select_sql(Some(&format!("user_name='{}'", auth.username))),
    )
    .await?;
    if let Some(uam_user) = uam_user {
        if uam_user.pswd_hash != auth.password {
            return Err(ApiError::Unauthorized("Invalid password".to_string()));
        }

        let user = auth_user::User::new(uam_user.id);

        let token = jwt.encode(user)?;

        session.insert("token", token.clone()).await?;

        return Ok(Json(format!(
            "Authenticated user: {}, password: {}, token: {}",
            auth.username, auth.password, token
        )));
    } else {
        return Err(ApiError::Unauthorized("User not found".to_string()));
    }
}

#[derive(Deserialize)]
pub struct AuthUserVo {
    username: String,
    password: String,
    email: String,
}

pub async fn create_user(
    State(app_state): State<AppState>,
    Form(auth): Form<AuthUserVo>,
) -> Result<Json<String>, ApiError> {
    let new_user = UamUser {
        id: xid::new().to_string(),
        user_name: auth.username,
        pswd_hash: auth.password,
        email: auth.email,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let inserted_user = create(&app_state.db, &new_user.insert_sql()).await?;

    Ok(Json(format!(
        "Created user: {}, id: {}",
        new_user.user_name,
        inserted_user.last_insert_rowid()
    )))
}
