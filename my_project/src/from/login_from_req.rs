use axum::extract::FromRequestParts;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::entities::uam_role::UamRole;
use crate::repositories::{read, read_one};
use crate::sitemaps::app_state::AppState;
use crate::{
    api_base::api_errors::ApiError,
    auth::{auth_perm, auth_user},
    entities::uam_user::UamUser,
};

#[derive(Debug)]
pub struct LoginUser(pub auth_user::User);

impl FromRequestParts<AppState> for LoginUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let perm = parts
            .extensions
            .get::<auth_perm::Permission>()
            .cloned()
            .ok_or_else(|| ApiError::Unauthorized("No Authorized".to_string()))?;

        let db = state.db.clone();

        let user_id = format!("'{}'", perm.get_id().unwrap());

        let query_user = UamUser::select_sql(Some(&UamUser::where_eq("id", &user_id)));

        let user = read_one::<&SqlitePool, UamUser>(&db, &query_user).await?;

        let query_role = UamRole::select_sql(Some(&UamRole::where_eq("user_id", &user_id)));

        let roles = read::<&SqlitePool, UamRole>(&db, &query_role).await?;

        let mut auth_user = auth_user::User::new(perm.get_id().unwrap());
        if let Some(user) = user {
            auth_user.username = Some(user.user_name);
            auth_user.first_name = Some("".to_string());
            auth_user.last_name = Some("".to_string());
            auth_user.email = Some(user.email);
            auth_user.date_joined = Some(Utc::now());
            auth_user.is_active = Some(true);
            auth_user.is_superuser = Some(true);
            auth_user.groups = Some(roles.iter().map(|r| r.code.clone()).collect());
        }

        Ok(LoginUser(auth_user))
    }
}
