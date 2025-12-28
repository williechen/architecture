use std::collections::HashSet;

use architecture::{
    configures,
    entities::{batches, order_lines},
    repositories::create,
};
use axum::{body::Body, extract::Request};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use http_body_util::BodyExt;
use sqlx::{Sqlite, Transaction};
use tower::ServiceExt;

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
                &early_batch_ref,
                &sku,
                100,
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2011, 1, 1).unwrap(),
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ),
            (
                &later_batch_ref,
                &sku,
                100,
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2011, 1, 2).unwrap(),
                    chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                )),
            ),
            (&other_batch_ref, &other_sku, 100, None),
        ],
    )
    .await;

    tx.commit().await.unwrap();

    let data = order_lines::OrderLine {
        id: random_order_id(""),
        sku: sku.clone(),
        qty: 3,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let route = architecture::sitemaps::sitemap(db).await;

    let request = Request::builder()
        .method("POST")
        .uri("/allocate")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap();

    let res = route.oneshot(request).await.unwrap();

    let status = res.status();
    assert_eq!(status, 201);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let body_str = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    let batch_ref = body_str.get("batch_ref").unwrap().as_str().unwrap();
    assert_eq!(batch_ref, early_batch_ref);
}

#[tokio::test]
async fn test_400_message_for_invalid_sku() {
    let unknown_sku = random_sku("1");
    let order_id = random_order_id("");

    let db = configures::AppConfig::load()
        .database
        .get_connection()
        .await;

    let data = order_lines::OrderLine {
        id: order_id.clone(),
        sku: unknown_sku.clone(),
        qty: 20,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let route = architecture::sitemaps::sitemap(db).await;

    let request = Request::builder()
        .method("POST")
        .uri("/allocate")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap();

    let response = route.clone().oneshot(request).await.unwrap();
    let status = response.status();
    assert_eq!(status, 400);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    let message = body_str.get("message").unwrap().as_str().unwrap();
    assert_eq!(message, format!("Invalid sku {}", unknown_sku));
}
