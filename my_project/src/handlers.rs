use std::str::FromStr;

use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use sqlx::SqliteConnection;

use crate::entities::batches::Batch;
use crate::entities::products::Product;
use crate::repositories::{create, read, read_one};
use crate::{chapter1, events};

pub async fn add_batch(
    event: events::BatchCreate,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<(), sqlx::Error> {
    let db = &mut **tx;

    let where_clause_string = format!("sku = '{}'", event.sku);
    let where_clause = Some(where_clause_string.as_str());

    let product_ent =
        read_one::<&mut SqliteConnection, Product>(db, &Product::select_sql(where_clause)).await?;

    if let Some(ent) = product_ent {
        let new_batch = chapter1::Batch::new(&event.references, &event.sku, event.qty, event.eta);
        let _product = ent.build(vec![new_batch]);

        let batch_ent = Batch {
            id: xid::new().to_string(),
            reference: event.references.to_string(),
            sku: event.sku.to_string(),
            qty: event.qty,
            eta: event.eta,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        create::<&mut SqliteConnection>(db, &batch_ent.insert_sql()).await?;
    } else {
        let new_batch = chapter1::Batch::new(&event.references, &event.sku, event.qty, event.eta);
        let _product = chapter1::Product::new(&event.sku, vec![new_batch]);

        let ent = Product {
            id: xid::new().to_string(),
            sku: event.sku.to_string(),
            version_number: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        create::<&mut SqliteConnection>(db, &ent.insert_sql()).await?;

        let batch_ent = Batch {
            id: xid::new().to_string(),
            reference: event.references.to_string(),
            sku: event.sku.to_string(),
            qty: event.qty,
            eta: event.eta,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        create::<&mut SqliteConnection>(db, &batch_ent.insert_sql()).await?;
    }

    Ok(())
}

pub async fn allocate(
    event: events::AllocateRequired,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<Option<(String, i32)>, String> {
    let db = &mut **tx;

    let order = chapter1::OrderLine {
        order_id: event.order_id.to_string(),
        sku: event.sku.to_string(),
        qty: event.qty,
    };

    let where_clause_string = format!("sku = '{}'", event.sku);
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
        return Err(format!("Invalid sku {}", event.sku));
    }
}

pub async fn send_out_of_stock_notification(
    event: events::OutOfStock,
    _tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder for sending out of stock notification

    let creds = Credentials::new(
        "".to_string(),
        "".to_string(), // Gmail 要用應用程式密碼
    );

    // 建立郵件傳輸器
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let from = Mailbox::from_str("").unwrap();
    let to = Mailbox::from_str("").unwrap();

    mailer.send(
        &lettre::Message::builder()
            .from(from)
            .to(to)
            .subject("Out of Stock Notification")
            .body(format!("The item with SKU {} is out of stock.", event.sku))?,
    )?;

    println!("Out of stock notification sent for SKU: {}", event.sku);
    Ok(())
}
