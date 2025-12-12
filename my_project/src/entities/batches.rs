use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, SqlTable)]
pub struct Batch {
    pub id: String,
    pub reference: String,
    pub sku: String,
    pub qty: u32,
    pub eta: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
