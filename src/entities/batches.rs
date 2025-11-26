use rbatis::crud;
use rbatis::rbdc::DateTime; // Import the macro

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Batch {
    pub id: String,
    pub reference: String,
    pub sku: String,
    pub purchased_quantity: i32,
    pub eta: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(Batch {}, "batches");

impl Default for Batch {
    fn default() -> Self {
        Batch {
            id: "".to_string(),
            reference: "".to_string(),
            sku: "".to_string(),
            purchased_quantity: 0,
            eta: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
