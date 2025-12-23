use std::process::Command;

use crate::{
    entities::ssm_codemap::SsmCodemap, repositories::create, sitemaps::app_state::AppState,
};
use axum::{Json, Router, extract::State, routing::get};
use chrono::Local;

pub fn common_routes() -> Router<AppState> {
    Router::new()
        .route("/cache", get(get_cache))
        .route("/health", get(health_check))
}

async fn get_cache(State(state): State<AppState>) -> Json<String> {
    let codemap = SsmCodemap {
        id: xid::new().to_string(),
        category: "example".to_string(),
        code: "001".to_string(),
        name: "Example Code".to_string(),
        description: "This is an example code entry.".to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };
    create(&state.db, &codemap.insert_sql()).await.unwrap();

    let data = state.codemap.read().await;
    let json_str = serde_json::to_string(&*data).unwrap_or_else(|_| "{}".to_string());
    Json(json_str)
}

async fn health_check() -> Json<String> {
    // -------------------------
    // 1. Semantic Version
    // -------------------------
    // Priority:
    //   CI_VERSION > GIT_TAG > fallback to CARGO_PKG_VERSION
    let semver = std::env::var("CI_VERSION")
        .or_else(|_| std::env::var("GIT_TAG"))
        .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string());

    // -------------------------
    // 2. Timestamp (YYYYMMDD.HHMMSS)
    // -------------------------
    let timestamp = Local::now().format("%Y%m%d.%H%M%S").to_string();

    // -------------------------
    // 3. Build Number (optional)
    // -------------------------
    let build_number = std::env::var("BUILD_NUMBER").unwrap_or_else(|_| "0".to_string());

    // -------------------------
    // 4. Git Commit Hash
    // -------------------------
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "nogit".to_string());

    // -------------------------
    // 5. Compose final version
    //    semver+timestamp.buildX.githash
    // -------------------------
    let full_version = format!(
        "{}+{}.build{}.g{}",
        semver, timestamp, build_number, git_hash,
    );

    Json(full_version)
}
