use std::collections::HashSet;

use architecture::{
    configures,
    entities::{batches, order_lines},
    repositories::create,
};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::{Sqlite, Transaction};

fn random_suffix() -> String {
    xid::new().to_string()[..6].to_string()
}

fn random_sku(name: &str) -> String {
    format!("sku-{}-{}", name, random_suffix())
}

fn random_batch_ref(name: &str) -> String {
    format!("batch-{}-{}", name, random_suffix())
}

fn random_order_id(name: &str) -> String {
    format!("order-{}-{}", name, random_suffix())
}

async fn add_stock(
    db: &mut Transaction<'_, Sqlite>,
    datas: Vec<(&str, &str, u32, Option<NaiveDateTime>)>,
) -> (HashSet<String>, HashSet<String>) {
    let mut skus = HashSet::new();
    let mut batch_refs = HashSet::new();

    for (batch_ref, sku, qty, eta) in datas {
        let data = batches::Batch {
            id: xid::new().to_string(),
            reference: batch_ref.to_string(),
            sku: sku.to_string(),
            qty,
            eta: eta.map(|d| d),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        create(&mut **db, &data.insert_sql()).await.unwrap();

        skus.insert(sku.to_string());
        batch_refs.insert(data.id.clone());
    }

    (skus, batch_refs)
}

#[tokio::test]
async fn test_api_returns_allocation() {
    let db = configures::AppConfig::load()
        .database
        .get_connection()
        .await;

    let mut tx = db.begin().await.unwrap();

    let sku = random_sku("");
    let other_sku = random_sku("OTHER");

    let early_batch_ref = random_batch_ref("1");
    let later_batch_ref = random_batch_ref("2");
    let other_batch_ref = random_batch_ref("3");

    add_stock(
        &mut tx,
        vec![
            (
                &later_batch_ref,
                &sku,
                100,
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2012, 1, 1).unwrap(),
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ),
            (
                &early_batch_ref,
                &sku,
                100,
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2011, 1, 1).unwrap(),
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ),
            (&other_batch_ref, &other_sku, 100, None),
        ],
    )
    .await;

    let data = order_lines::OrderLine {
        id: xid::new().to_string(),
        order_id: random_order_id(""),
        sku: sku.clone(),
        qty: 3,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let req = reqwest::Client::new()
        .post("http://localhost:3000/allocate")
        .json(&data)
        .send()
        .await
        .unwrap();

    assert_eq!(req.status(), 201);
    let batch_ref_resp: String = req.json().await.unwrap();
    assert_eq!(batch_ref_resp, early_batch_ref);
}
