use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, SqlTable)]
pub struct SsmCodemap {
    pub id: String,
    pub category: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
