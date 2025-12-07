use rbatis::{crud, rbdc::DateTime};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UamUserRole {
    pub id: String,
    pub uam_user_id: String,
    pub uam_role_id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(UamUserRole {}, "uam_user_role");

impl Default for UamUserRole {
    fn default() -> Self {
        UamUserRole {
            id: "".to_string(),
            uam_user_id: "".to_string(),
            uam_role_id: "".to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
