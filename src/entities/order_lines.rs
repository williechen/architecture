use chrono::{Local, NaiveDateTime};
use rbatis::{crud, impl_select, rbdc::db::ExecResult};

use crate::chapter1::model; // Import the crud macro

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    pub id: String,
    pub order_id: String,
    pub sku: String,
    pub qty: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

crud!(OrderLine {}, "order_lines");
impl_select!(OrderLine{select_by_order_id(order_id:&str) => "`where order_id = #{order_id}`"}, "order_lines");

impl Default for OrderLine {
    fn default() -> Self {
        OrderLine {
            id: "".to_string(),
            order_id: "".to_string(),
            sku: "".to_string(),
            qty: 0,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

impl OrderLine {
    pub fn new(id: String, order_id: String, sku: String, qty: u32) -> Self {
        OrderLine {
            id,
            order_id,
            sku,
            qty,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }

    pub async fn find_all(db: &rbatis::RBatis) -> rbatis::Result<Vec<model::OrderLine>> {
        let order_lines = OrderLine::select_all(db).await;
        let order_line_models: Vec<model::OrderLine> = match order_lines {
            Ok(lines) => lines
                .into_iter()
                .map(|ol| model::OrderLine {
                    order_id: ol.order_id,
                    sku: ol.sku,
                    qty: ol.qty,
                })
                .collect(),
            Err(_) => vec![],
        };

        Ok(order_line_models)
    }

    pub async fn get(db: &rbatis::RBatis, id: &str) -> rbatis::Result<Option<model::OrderLine>> {
        let order_lines = OrderLine::select_by_map(db, rbs::Value::from(id)).await;
        match order_lines {
            Ok(list) => {
                if list.is_empty() {
                    Ok(None)
                } else {
                    let ol = &list[0];
                    Ok(Some(model::OrderLine {
                        order_id: ol.order_id.clone(),
                        sku: ol.sku.clone(),
                        qty: ol.qty,
                    }))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn create(
        db: &rbatis::RBatis,
        order_line: &model::OrderLine,
    ) -> rbatis::Result<String> {
        let order_line = OrderLine {
            id: xid::new().to_string(),
            order_id: order_line.order_id.clone(),
            sku: order_line.sku.clone(),
            qty: order_line.qty,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        };
        OrderLine::insert(db, &order_line).await?;

        Ok(order_line.id)
    }

    pub async fn modify(
        db: &rbatis::RBatis,
        order_line: &model::OrderLine,
        id: &str,
    ) -> rbatis::Result<ExecResult> {
        let order_line = OrderLine {
            id: id.to_string(),
            order_id: order_line.order_id.clone(),
            sku: order_line.sku.clone(),
            qty: order_line.qty,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        };
        OrderLine::update_by_map(db, &order_line, rbs::Value::from(id)).await
    }

    pub async fn remove(db: &rbatis::RBatis, id: &str) -> rbatis::Result<ExecResult> {
        OrderLine::delete_by_map(db, rbs::Value::from(id)).await
    }
}
