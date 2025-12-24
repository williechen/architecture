use chrono::NaiveDateTime;
use sql_derives::SqlTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, SqlTable)]
pub struct UamUserRole {
    pub id: String,
    pub uam_user_id: String,
    pub uam_role_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
