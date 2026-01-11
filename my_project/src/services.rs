use chrono::{DateTime, Utc};
use sqlx::{SqliteConnection, Transaction};

use crate::{
    chapter1,
    entities::{batches::Batch, products::Product},
    repositories::{create, read, read_one},
};

pub async fn allocate(
    order_id: &str,
    sku: &str,
    qty: u32,
    tx: &mut Transaction<'_, sqlx::Sqlite>,
) -> Result<Option<(String, i32)>, String> {
    let db = &mut **tx;

    let order = chapter1::OrderLine {
        order_id: order_id.to_string(),
        sku: sku.to_string(),
        qty,
    };

    let where_clause_string = format!("sku = '{}'", sku);
    let where_clause = Some(where_clause_string.as_str());

    let product_ent =
        read_one::<&mut SqliteConnection, Product>(db, &Product::select_sql(where_clause))
            .await
            .unwrap();
    if let Some(ent) = product_ent {
        let batche_ents =
            read::<&mut SqliteConnection, Batch>(db, &Batch::select_sql(where_clause))
                .await
                .unwrap();

        let batches = batche_ents
            .into_iter()
            .map(|b| b.build())
            .collect::<Vec<chapter1::Batch>>();

        let res = ent.build(batches).allocate(&order);
        return res;
    } else {
        return Err(format!("Invalid sku {}", sku));
    }
}

pub async fn add_batch(
    reference: &str,
    sku: &str,
    quantity: u32,
    eta: Option<DateTime<Utc>>,
    tx: &mut Transaction<'_, sqlx::Sqlite>,
) -> Result<(), sqlx::Error> {
    let db = &mut **tx;

    let where_clause_string = format!("sku = '{}'", sku);
    let where_clause = Some(where_clause_string.as_str());

    let product_ent =
        read_one::<&mut SqliteConnection, Product>(db, &Product::select_sql(where_clause)).await?;

    if let Some(ent) = product_ent {
        let new_batch = chapter1::Batch::new(reference, sku, quantity, eta);
        let _product = ent.build(vec![new_batch]);

        let batch_ent = Batch {
            id: xid::new().to_string(),
            reference: reference.to_string(),
            sku: sku.to_string(),
            qty: quantity,
            eta,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        create::<&mut SqliteConnection>(db, &batch_ent.insert_sql()).await?;
    } else {
        let new_batch = chapter1::Batch::new(reference, sku, quantity, eta);
        let _product = chapter1::Product::new(sku, vec![new_batch]);

        let ent = Product {
            id: xid::new().to_string(),
            sku: sku.to_string(),
            version_number: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        create::<&mut SqliteConnection>(db, &ent.insert_sql()).await?;

        let batch_ent = Batch {
            id: xid::new().to_string(),
            reference: reference.to_string(),
            sku: sku.to_string(),
            qty: quantity,
            eta,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        create::<&mut SqliteConnection>(db, &batch_ent.insert_sql()).await?;
    }

    Ok(())
}
