use architecture::{configures, entities::order_lines};
use axum::{body::Body, extract::Request};
use chrono::Utc;
use http_body_util::BodyExt;
use serde_json::Value;
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

async fn post_to_add_batch(refe: &str, sku: &str, qty: u32, eta: Option<String>) {
    let db = configures::AppConfig::load()
        .database
        .get_connection()
        .await;
    let route = architecture::sitemaps::sitemap(db).await;

    let mut map = serde_json::Map::new();
    map.insert("reference".to_string(), Value::String(refe.to_string()));
    map.insert("sku".to_string(), Value::String(sku.to_string()));
    map.insert("qty".to_string(), Value::Number(qty.into()));
    if let Some(e) = eta {
        map.insert("eta".to_string(), Value::String(e.to_string()));
    } else {
        map.insert("eta".to_string(), Value::Null);
    }

    let request = Request::builder()
        .method("POST")
        .uri("/add_batch")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&map).unwrap()))
        .unwrap();

    let res = route.oneshot(request).await.unwrap();

    let status = res.status();
    assert_eq!(status, 201);
}

#[tokio::test]
async fn test_api_returns_allocation() {
    let db = configures::AppConfig::load()
        .database
        .get_connection()
        .await;

    let sku = random_sku("");
    let other_sku = random_sku("OTHER");

    let early_batch_ref = random_batch_ref("1");
    post_to_add_batch(&early_batch_ref, &sku, 100, Some("2011-01-02".to_string())).await;
    let later_batch_ref = random_batch_ref("2");
    post_to_add_batch(&later_batch_ref, &sku, 100, Some("2011-01-01".to_string())).await;
    let other_batch_ref = random_batch_ref("3");
    post_to_add_batch(&other_batch_ref, &other_sku, 100, None).await;

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
