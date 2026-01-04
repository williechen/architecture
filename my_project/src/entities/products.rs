use chrono::NaiveDateTime;
use sql_derives::SqlTable;

use crate::chapter1;

#[derive(Clone, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub version_number: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Product {
    pub fn build(&self, batches: Vec<chapter1::Batch>) -> chapter1::Product {
        chapter1::Product {
            sku: self.sku.clone(),
            version_number: self.version_number + 1,
            batches: batches,
        }
    }
}
