use chrono::{DateTime, Utc};
use sql_derives::SqlTable; // Import the macro

#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow,
)]
pub struct Allocation {
    pub id: String,
    pub batch_id: String,
    pub order_line_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
