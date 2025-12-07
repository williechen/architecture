use rbatis::{crud, rbdc::DateTime};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UamRole {
    pub id: String,
    pub code: String,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(UamRole {}, "uam_role");

impl Default for UamRole {
    fn default() -> Self {
        UamRole {
            id: "".to_string(),
            code: "".to_string(),
            name: "".to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
