use chrono::{DateTime, Utc};

pub enum Event {
    BatchCreate(BatchCreate),
    AllocateRequired(AllocateRequired),
    OutOfStock(OutOfStock),
}

pub struct BatchCreate {
    pub references: String,
    pub sku: String,
    pub qty: u32,
    pub eta: Option<DateTime<Utc>>,
}

pub struct AllocateRequired {
    pub order_id: String,
    pub sku: String,
    pub qty: u32,
}

pub struct OutOfStock {
    pub sku: String,
}
