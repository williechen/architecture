use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select}; // Import the macro

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
impl_select!(Batch{select_by_reference(reference:&str) => "`where reference = #{reference}`"});

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

impl Batch {
    pub fn new(reference: &str, sku: &str, qrt: i32, eta: Option<DateTime>) -> Self {
        Batch {
            id: xid::new().to_string(),
            reference: reference.to_string(),
            sku: sku.to_string(),
            purchased_quantity: qrt,
            eta,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
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

    pub async fn get<T>(db: &rbatis::RBatis, id: T) -> rbatis::Result<Option<model::Batch>> {
        let batches = Batch::select_by_map(db, rbs::value::Value::from(id)).await;
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
            purchased_quantity: batch::available_quantity(),
            eta: batch.eta,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        Batch::insert(db, batch).await;

        Ok(batch.id)
    }

    pub async fn modify<T>(
        db: &rbatis::RBatis,
        batch: &model::Batch,
        id: T,
    ) -> rbatis::Result<u64> {
        let batch = Batch {
            id,
            reference: batch.reference.clone(),
            sku: batch.sku.clone(),
            purchased_quantity: batch.purchased_quantity,
            eta: batch.eta,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };
        Batch::update_by_map(db, &batch, rbs::value::Value::from(id)).await
    }

    pub async fn remove<T>(db: &rbatis::RBatis, id: T) -> rbatis::Result<u64> {
        Batch::delete_by_map(db, rbs::value::Value::from(id)).await
    }
}
