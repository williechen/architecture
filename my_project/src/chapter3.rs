use std::collections::HashMap;

use crate::{
    api_base::api_errors::ApiError, repositories::update, services, sitemaps::app_state::AppState,
};
use axum::{
    Json, Router, debug_handler, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};
use chrono::NaiveDateTime;
use sqlx::SqliteConnection;

pub fn logic_routes() -> Router<AppState> {
    Router::new()
        .route("/allocate", post(allocate_handler))
        .route("/add_batch", post(add_batch_handler))
}

#[derive(serde::Deserialize)]
pub struct AllocateReq {
    pub id: String,
    pub sku: String,
    pub qty: u32,
}

#[debug_handler]
pub async fn allocate_handler(
    State(app_state): State<AppState>,
    Json(req): Json<AllocateReq>,
) -> Result<impl IntoResponse, ApiError> {
    let db = &app_state.db;
    let mut tx = db.begin().await.unwrap();

    let allocate = services::allocate(&req.id, &req.sku, req.qty, &mut tx).await;
    match allocate {
        Ok(option) => {
            if let Some(batch_ref) = option {
                update::<&mut SqliteConnection>(
                    &mut *tx,
                    &format!(
                        "UPDATE batch SET qty = qty - {} WHERE reference = '{}'",
                        req.qty, batch_ref
                    ),
                )
                .await
                .unwrap();

                tx.commit().await.unwrap();

                return Ok((
                    StatusCode::CREATED,
                    Json({
                        let mut res = HashMap::new();
                        res.insert("batch_ref", batch_ref);
                        res
                    }),
                ));
            } else {
                tx.rollback().await.unwrap();

                return Err(ApiError::BadRequest(format!(
                    "Out of stock for sku {}",
                    req.sku.clone()
                )));
            }
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            return Err(ApiError::BadRequest(e));
        }
    }
}

#[derive(serde::Deserialize)]
pub struct AddBatchReq {
    pub reference: String,
    pub sku: String,
    pub qty: u32,
    pub eta: Option<String>,
}

#[debug_handler]
pub async fn add_batch_handler(
    State(app_state): State<AppState>,
    Json(req): Json<AddBatchReq>,
) -> Result<impl IntoResponse, ApiError> {
    let db = &app_state.db;
    let mut tx = db.begin().await.unwrap();

    match services::add_batch(
        &req.reference,
        &req.sku,
        req.qty,
        req.eta
            .as_ref()
            .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d").ok()),
        &mut tx,
    )
    .await
    {
        Ok(_) => {
            tx.commit().await.unwrap();
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            return Err(ApiError::InternalServerError(format!(
                "Failed to add batch: {}",
                e
            )));
        }
    }

    Ok((StatusCode::CREATED, "").into_response())
}
