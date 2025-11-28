use chrono::{Local, NaiveDateTime};
use rbatis::{crud, impl_select, rbdc::db::ExecResult};

use crate::chapter1::model; // Import the macro

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Batch {
    pub id: String,
    pub reference: String,
    pub sku: String,
    pub purchased_quantity: u32,
    pub eta: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

crud!(Batch {}, "batches");
impl_select!(Batch{select_by_reference(reference:&str) => "`where reference = #{reference}`"}, "batches");

impl Default for Batch {
    fn default() -> Self {
        Batch {
            id: "".to_string(),
            reference: "".to_string(),
            sku: "".to_string(),
            purchased_quantity: 0,
            eta: None,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

impl Batch {
    pub fn new(reference: &str, sku: &str, qrt: u32, eta: Option<NaiveDateTime>) -> Self {
        Batch {
            id: xid::new().to_string(),
            reference: reference.to_string(),
            sku: sku.to_string(),
            purchased_quantity: qrt,
            eta,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }

    pub async fn find_all(db: &rbatis::RBatis) -> rbatis::Result<Vec<model::Batch>> {
        let batches = Batch::select_all(db).await?;
        let batch_models: Vec<model::Batch> = batches
            .into_iter()
            .map(|b| model::Batch::new(&b.reference, &b.sku, b.purchased_quantity, b.eta))
            .collect();
        Ok(batch_models)
    }

    pub async fn get(db: &rbatis::RBatis, id: &str) -> rbatis::Result<Option<model::Batch>> {
        let batches = Batch::select_by_map(db, rbs::Value::from(id)).await;
        match batches {
            Ok(list) => {
                if list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(model::Batch::new(
                        &list[0].reference,
                        &list[0].sku,
                        list[0].purchased_quantity,
                        list[0].eta,
                    )))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn create(db: &rbatis::RBatis, batch: &model::Batch) -> rbatis::Result<String> {
        let batch = Batch {
            id: xid::new().to_string(),
            reference: batch.reference.clone(),
            sku: batch.sku.clone(),
            purchased_quantity: batch.available_quantity(),
            eta: batch.eta,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        };

        Batch::insert(db, &batch).await?;

        Ok(batch.id)
    }

    pub async fn modify(
        db: &rbatis::RBatis,
        batch: &model::Batch,
        id: &str,
    ) -> rbatis::Result<ExecResult> {
        let batch = Batch {
            id: id.to_string(),
            reference: batch.reference.clone(),
            sku: batch.sku.clone(),
            purchased_quantity: batch.available_quantity(),
            eta: batch.eta,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        };
        Batch::update_by_map(db, &batch, rbs::Value::from(id)).await
    }

    pub async fn remove(db: &rbatis::RBatis, id: &str) -> rbatis::Result<ExecResult> {
        Batch::delete_by_map(db, rbs::Value::from(id)).await
    }
}
