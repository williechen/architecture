use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow,
)]
#[sql(table = "order_lines")]
pub struct OrderLine {
    pub id: String,
    pub order_id: String,
    pub sku: String,
    pub qty: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
