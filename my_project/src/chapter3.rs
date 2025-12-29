use std::collections::HashMap;

use crate::{
    api_base::api_errors::ApiError,
    chapter1,
    entities::batches,
    repositories::{create, read, update},
    services,
    sitemaps::app_state::AppState,
};
use axum::{
    Json, Router, debug_handler, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};
use chrono::{NaiveDateTime, Utc};
use sqlx::SqlitePool;

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

    let batches =
        read::<&SqlitePool, batches::Batch>(db, &batches::Batch::select_sql(None)).await?;

    let mut built_batches: Vec<chapter1::Batch> = batches.into_iter().map(|b| b.build()).collect();
    let batch_vos: Vec<&mut chapter1::Batch> = built_batches.iter_mut().collect();

    let order_line = chapter1::OrderLine {
        order_id: req.id.clone(),
        sku: req.sku.clone(),
        qty: req.qty,
    };

    let allocate = services::allocate(&order_line, batch_vos);

    match allocate {
        Ok(option) => {
            if let Some(batch_ref) = option {
                update::<&SqlitePool>(
                    db,
                    &format!(
                        "UPDATE batch SET qty = qty - {} WHERE reference = '{}'",
                        req.qty, batch_ref
                    ),
                )
                .await
                .unwrap();

                return Ok((
                    StatusCode::CREATED,
                    Json({
                        let mut res = HashMap::new();
                        res.insert("batch_ref", batch_ref);
                        res
                    }),
                ));
            } else {
                return Err(ApiError::BadRequest(format!(
                    "Out of stock for sku {}",
                    req.sku.clone()
                )));
            }
        }
        Err(e) => {
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

    let eta = match req.eta {
        Some(ref eta_str) => {
            let mut eta_str = eta_str.clone();
            eta_str.push_str(" 00:00:00");
            Some(NaiveDateTime::parse_from_str(&eta_str, "%Y-%m-%d %H:%M:%S").unwrap())
        }
        None => None,
    };

    let batch = batches::Batch {
        id: xid::new().to_string(),
        reference: req.reference,
        sku: req.sku,
        qty: req.qty,
        eta,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    create::<&SqlitePool>(db, &batch.insert_sql()).await?;

    Ok((StatusCode::CREATED, Json(batch)).into_response())
}
