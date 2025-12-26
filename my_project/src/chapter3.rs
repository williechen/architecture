use std::collections::HashMap;

use crate::{
    api_base::api_errors::ApiError,
    chapter1,
    entities::batches,
    repositories::{read, update},
    sitemaps::app_state::AppState,
};
use axum::{
    Json, Router, debug_handler, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};
use sqlx::SqlitePool;

pub fn logic_routes() -> Router<AppState> {
    Router::new().route("/allocate", post(allocate_handler))
}

#[derive(serde::Deserialize)]
pub struct AllocateVo {
    pub order_id: String,
    pub sku: String,
    pub qty: u32,
}

#[debug_handler]
pub async fn allocate_handler(
    State(app_state): State<AppState>,
    Json(vo): Json<AllocateVo>,
) -> Result<impl IntoResponse, ApiError> {
    let db = &app_state.db;

    let batches =
        read::<&SqlitePool, batches::Batch>(db, &batches::Batch::select_sql(None)).await?;

    let mut batch_vos: Vec<chapter1::Batch> = batches
        .into_iter()
        .map(|b| chapter1::Batch::new(&b.reference, &b.sku, b.qty, b.eta))
        .collect();

    let exists_sku = batch_vos.iter().any(|b| b.sku == vo.sku);
    if !exists_sku {
        return Err(ApiError::BadRequest(format!("Invalid SKU {}", vo.sku)));
    }

    let batch_refs: Vec<&mut chapter1::Batch> = batch_vos.iter_mut().collect();

    let order_line = chapter1::OrderLine {
        order_id: vo.order_id.clone(),
        sku: vo.sku.clone(),
        qty: vo.qty,
    };

    let allocate = chapter1::allocate(&order_line, batch_refs);

    match allocate {
        Ok(option) => {
            if let Some(batch_ref) = option {
                update::<&SqlitePool>(
                db,
                &format!(
                    "UPDATE batches SET purchased_quantity = purchased_quantity - {} WHERE reference = '{}'",
                    vo.qty, batch_ref
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
                    vo.sku.clone()
                )));
            }
        }
        Err(e) => {
            return Err(ApiError::BadRequest(e));
        }
    }
}
