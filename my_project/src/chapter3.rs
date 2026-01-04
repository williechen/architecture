use std::collections::HashMap;

use crate::{
    api_base::api_errors::ApiError,
    repositories::{read_one, update},
    services,
    sitemaps::app_state::AppState,
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
            let version_number: Option<(i32,)> = read_one::<&mut SqliteConnection, (i32,)>(
                &mut *tx,
                &format!(
                    "SELECT MAX(version_number) FROM product WHERE sku = '{}'",
                    req.sku
                ),
            )
            .await
            .unwrap();

            if let Some(batch_ref) = option {
                // 檢查版本號是否衝突
                if let Some(version_number) = version_number {
                    if version_number.0 <= batch_ref.1 {
                        tx.rollback().await.unwrap();
                        return Err(ApiError::BadRequest(format!(
                            "Version number conflict for sku {}",
                            req.sku
                        )));
                    }
                }

                update::<&mut SqliteConnection>(
                    &mut *tx,
                    &format!(
                        "UPDATE product SET version_number = {} WHERE sku = '{}'",
                        batch_ref.1, req.sku
                    ),
                )
                .await
                .unwrap();

                update::<&mut SqliteConnection>(
                    &mut *tx,
                    &format!(
                        "UPDATE batch SET qty = qty - {} WHERE reference = '{}'",
                        req.qty, batch_ref.0
                    ),
                )
                .await
                .unwrap();

                tx.commit().await.unwrap();

                return Ok((
                    StatusCode::CREATED,
                    Json({
                        let mut res = HashMap::new();
                        res.insert("batch_ref", batch_ref.0);
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
