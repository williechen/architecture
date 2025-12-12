use chrono::NaiveDateTime;
use sql_derives::SqlTable; // Import the macro

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, SqlTable)]
pub struct Allocation {
    pub id: String,
    pub batch_id: String,
    pub order_line_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
