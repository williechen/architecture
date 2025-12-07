use rbatis::{crud, rbdc::DateTime};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UamUser {
    pub id: String,
    pub user_name: String,
    pub pswd_hash: String,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(UamUser {}, "uam_user");

impl Default for UamUser {
    fn default() -> Self {
        UamUser {
            id: "".to_string(),
            user_name: "".to_string(),
            pswd_hash: "".to_string(),
            email: "".to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
