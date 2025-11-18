use rbatis::{crud, rbdc::DateTime};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SsmConfig {
    pub id: String,
    pub category: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(SsmConfig {}, "ssm_config");

impl Default for SsmConfig {
    fn default() -> Self {
        SsmConfig {
            id: "".to_string(),
            category: "".to_string(),
            code: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
