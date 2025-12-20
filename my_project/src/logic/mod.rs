mod authenticator;
pub mod common;
pub mod security_from_req;

use askama::Template;
use axum::Router;
use axum::response::Html;
use axum::routing::get;

use crate::logic::authenticator::authenticator_layer;
use crate::sitemaps::app_state::AppState;
use crate::web_base::web_errors::WebError;

pub async fn logic_routes() -> Router<AppState> {
    let config = crate::tokens::jwt::JwtConfig::default();
    let skip_paths = vec![
        String::from("/login"),
        String::from("/static/*"),
        String::from("/plugins/*"),
    ];

    Router::new()
        .route("/", get(home))
        .route_layer(authenticator_layer(config, skip_paths))
        .route("/login", get(login))
}

pub async fn home() -> Result<Html<String>, WebError> {
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct HomeTemplate {}

    let template = HomeTemplate {};
    Ok(Html(template.render()?))
}

pub struct SecurityData {
    pub username: String,
    pub password: String,
}

pub async fn login() -> Result<Html<String>, WebError> {
    #[derive(Debug, Template)]
    #[template(path = "login.html")]
    struct LoginTemplate {}

    let template = LoginTemplate {};
    Ok(Html(template.render()?))
}
