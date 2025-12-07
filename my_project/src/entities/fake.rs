use std::collections::HashSet;

use crate::{chapter1::model, configures::AppConfig, entities::batches::Batch};

pub struct Fake {
    _batches: HashSet<model::Batch>,
}

impl Fake {
    pub async fn add(&self, batch: model::Batch) -> Result<String, rbatis::Error> {
        let db = AppConfig::load().database.get_connection().await;

        Ok(Batch::create(&db, &batch).await?)
    }

    pub async fn get(&self, reference: String) -> Result<Option<model::Batch>, rbatis::Error> {
        let db = AppConfig::load().database.get_connection().await;

        let batch = Batch::select_by_reference(&db, &reference).await?;
        match batch.into_iter().next() {
            Some(b) => Ok(Some(model::Batch::new(
                &b.reference,
                &b.sku,
                b.purchased_quantity,
                b.eta,
            ))),
            None => Ok(None),
        }
    }
}
