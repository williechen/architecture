use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow,
)]
#[sql(table = "batches")]
pub struct Batch {
    pub id: String,
    pub reference: String,
    pub sku: String,
    pub qty: u32,
    pub eta: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
