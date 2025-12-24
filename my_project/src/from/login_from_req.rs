use axum::extract::{FromRequestParts, State};
use chrono::Utc;

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

        let user = read_one::<UamUser>(&db, &query_user).await?;

        let query_role = UamRole::select_sql(Some(&UamRole::where_eq("user_id", &user_id)));

        let roles = read::<UamRole>(&db, &query_role).await?;

        let mut authUser = auth_user::User::new(perm.get_id().unwrap());
        if let Some(user) = user {
            authUser.username = Some(user.user_name);
            authUser.first_name = Some("".to_string());
            authUser.last_name = Some("".to_string());
            authUser.email = Some(user.email);
            authUser.date_joined = Some(Utc::now());
            authUser.is_active = Some(true);
            authUser.is_superuser = Some(true);
            authUser.groups = Some(roles.iter().map(|r| r.code.clone()).collect());
        }

        Ok(LoginUser(authUser))
    }
}
