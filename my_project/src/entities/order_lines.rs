use chrono::{DateTime, Utc};
use sql_derives::SqlTable;

use crate::chapter1;

#[derive(Clone, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow)]
pub struct OrderLine {
    pub id: String,
    pub sku: String,
    pub qty: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OrderLine {
    pub fn build(&self) -> chapter1::OrderLine {
        chapter1::OrderLine {
            order_id: self.id.clone(),
            sku: self.sku.clone(),
            qty: self.qty,
        }
    }
}
