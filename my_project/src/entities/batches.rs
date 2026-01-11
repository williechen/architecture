use chrono::{DateTime, Utc};
use sql_derives::SqlTable;

use crate::{chapter1, entities::order_lines};

#[derive(Clone, serde::Serialize, serde::Deserialize, SqlTable, sqlx::FromRow)]
pub struct Batch {
    pub id: String,
    pub reference: String,
    pub sku: String,
    pub qty: u32,
    pub eta: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Batch {
    pub fn build(&self) -> chapter1::Batch {
        chapter1::Batch::new(&self.reference, &self.sku, self.qty, self.eta)
    }

    pub fn allocate(&self, line_ent: order_lines::OrderLine) -> chapter1::Batch {
        let mut batch = chapter1::Batch::new(&self.reference, &self.sku, self.qty, self.eta);

        let line = chapter1::OrderLine {
            order_id: line_ent.id.clone(),
            sku: line_ent.sku.clone(),
            qty: line_ent.qty,
        };

        batch.allocate(&line);
        batch
    }
}
