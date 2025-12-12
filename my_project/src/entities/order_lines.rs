use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, SqlTable)]
pub struct OrderLine {
    pub id: String,
    pub order_id: String,
    pub sku: String,
    pub qty: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
