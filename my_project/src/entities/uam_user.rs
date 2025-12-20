use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, SqlTable)]
pub struct UamUser {
    pub id: String,
    pub user_name: String,
    pub pswd_hash: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
