use chrono::NaiveDateTime;
use sql_derives::SqlTable; // Import the macro

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow,
)]
#[sql(table = "allocations")]
pub struct Allocation {
    pub id: String,
    pub batch_id: String,
    pub order_line_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
