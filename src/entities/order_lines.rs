use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select};

use crate::chapter1::model; // Import the crud macro

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    pub id: String,
    pub order_id: Option<String>,
    pub sku: Option<String>,
    pub qty: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(OrderLine {}, "order_lines");
impl_select!(OrderLine{select_by_order_id(order_id:&str) => "`where order_id = #{order_id}`"});

impl Default for OrderLine {
    fn default() -> Self {
        OrderLine {
            id: "".to_string(),
            order_id: None,
            sku: None,
            qty: 0,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl OrderLine {
    pub fn new(id: String, order_id: Option<String>, sku: Option<String>, qty: i32) -> Self {
        OrderLine {
            id,
            order_id,
            sku,
            qty,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub async fn find_all(db: &rbatis::Rbatis) -> rbatis::Result<Vec<model::OrderLine>> {
        let order_lines = OrderLine::select_all(db).await;
        let order_line_models: Vec<model::OrderLine> = match order_lines {
            Ok(lines) => lines
                .into_iter()
                .map(|ol| model::OrderLine::new(ol.order_id, ol.sku, ol.qty))
                .collect(),
            Err(_) => vec![],
        };

        Ok(order_line_models)
    }

    pub async fn get<T>(db: &rbatis::Rbatis, id: T) -> rbatis::Result<Option<model::OrderLine>> {
        let order_lines = OrderLine::select_by_map(db, rbs::value::Value::from(id)).await;
        match order_lines {
            Ok(list) => {
                if list.is_empty() {
                    Ok(None)
                } else {
                    let ol = &list[0];
                    Ok(Some(model::OrderLine::new(
                        ol.order_id.clone(),
                        ol.sku.clone(),
                        ol.qty,
                    )))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn create(
        db: &rbatis::Rbatis,
        order_line: &model::OrderLine,
    ) -> rbatis::Result<String> {
        let order_line = OrderLine {
            id: xid::new().to_string(),
            order_id: order_line.order_id.clone(),
            sku: order_line.sku.clone(),
            qty: order_line.qty,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };
        let result = OrderLine::insert(db, &order_line).await?;
        Ok(result)
    }

    pub async fn modify<T>(
        db: &rbatis::Rbatis,
        order_line: &model::OrderLine,
        id: T,
    ) -> rbatis::Result<u64> {
        let order_line = OrderLine {
            id,
            order_id: order_line.order_id.clone(),
            sku: order_line.sku.clone(),
            qty: order_line.qty,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };
        let result = OrderLine::update_by_map(db, &order_line, rbs::value::Value::from(id)).await?;
        Ok(result)
    }

    pub async fn remove<T>(db: &rbatis::Rbatis, id: T) -> rbatis::Result<u64> {
        let result = OrderLine::delete_by_map(db, rbs::value::Value::from(id)).await?;
        Ok(result)
    }
}
