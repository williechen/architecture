use rbatis::crud;
use rbatis::rbdc::DateTime; // Import the crud macro

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    pub id: String,
    pub order_id: Option<String>,
    pub sku: Option<String>,
    pub qty: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(OrderLine {}, "order_lines");

impl Default for OrderLine {
    fn default() -> Self {
        OrderLine {
            id: "".to_string(),
            order_id: None,
            sku: None,
            qty: 0,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
