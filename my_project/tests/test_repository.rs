use architecture::repositories::{create, read_one, read_to_json};
use architecture::{
    chapter1,
    entities::{allocations::Allocation, batches::Batch, order_lines::OrderLine},
};
use chrono::Local;
use serde_json::json;
use sqlx::Executor;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

async fn in_memory_db() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

async fn start_mappers(db: &SqlitePool) {
    db.execute(
        "CREATE TABLE test_table (
                id INTEGER PRIMARY KEY,
                name TEXT,
                age INTEGER,
                active INTEGER
            )",
    )
    .await
    .unwrap();

    db.execute(
        r"
        CREATE TABLE batches (
            id TEXT PRIMARY KEY,
            reference TEXT,
            sku TEXT,
            qty INTEGER,
            eta TEXT,
            created_at TEXT,
            updated_at TEXT
        )",
    )
    .await
    .unwrap();

    db.execute(
        r"
        CREATE TABLE order_lines (
            id TEXT PRIMARY KEY,
            order_id TEXT,
            sku TEXT,
            qty INTEGER,
            created_at TEXT,
            updated_at TEXT
        )",
    )
    .await
    .unwrap();

    db.execute(
        r"
        CREATE TABLE allocations (
            id TEXT PRIMARY KEY,
            order_line_id TEXT,
            batch_id TEXT,
            created_at TEXT,
            updated_at TEXT
        )",
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_repository_can_save_a_batch() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    let batch = Batch {
        reference: "batch1".to_string(),
        sku: "RUSTY-SOAPDISH".to_string(),
        qty: 100,
        eta: None,
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    create(&db, &batch.insert_sql()).await.unwrap();

    let fetched_batch = read_one::<&SqlitePool, Batch>(&db, &Batch::select_sql(None))
        .await
        .unwrap();

    let fetched_batch = fetched_batch.unwrap();
    assert_eq!(fetched_batch.reference, "batch1");
    assert_eq!(fetched_batch.sku, "RUSTY-SOAPDISH");
    assert_eq!(fetched_batch.qty, 100);
    assert_eq!(fetched_batch.eta, None);
}

#[tokio::test]
async fn test_repository_can_retrieve_a_batch_with_allocations() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    let order_line_id = insert_order_line(&db).await;
    let batch_id = insert_batch(&db, "batch1".to_string()).await;
    insert_batch(&db, "batch2".to_string()).await;
    insert_allocation(&db, order_line_id.clone(), batch_id.clone()).await;

    let fetched_batch = read_one::<&SqlitePool, Batch>(
        &db,
        &Batch::select_sql(Some(&format!("id = '{}'", batch_id))),
    )
    .await
    .unwrap();
    let fetched_order_line = read_one::<&SqlitePool, OrderLine>(
        &db,
        &OrderLine::select_sql(Some(&format!("id = '{}'", order_line_id))),
    )
    .await
    .unwrap();

    let expected = chapter1::Batch::new("batch1", "GENERIC-SOFA", 100, None);
    let expected_order = chapter1::OrderLine {
        order_id: "order1".to_string(),
        sku: "GENERIC-SOFA".to_string(),
        qty: 12,
    };

    let fetched_batch = fetched_batch.unwrap();
    let fetched_order_line = fetched_order_line.unwrap();
    assert_eq!(fetched_batch.reference, expected.reference);
    assert_eq!(fetched_batch.sku, expected.sku);
    assert_eq!(fetched_batch.qty, expected.available_quantity());

    assert_eq!(fetched_order_line.sku, expected_order.sku);
    assert_eq!(fetched_order_line.order_id, expected_order.order_id);
    assert_eq!(fetched_order_line.qty, expected_order.qty);
}

async fn insert_order_line(db: &SqlitePool) -> String {
    let order_line = OrderLine {
        order_id: "order1".to_string(),
        sku: "GENERIC-SOFA".to_string(),
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
        qty: 12,
    };

    create(db, &order_line.insert_sql()).await.unwrap();
    order_line.id
}

async fn insert_batch(db: &SqlitePool, batch_id: String) -> String {
    let batch = Batch {
        reference: batch_id,
        sku: "GENERIC-SOFA".to_string(),
        qty: 100,
        eta: None,
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    create(db, &batch.insert_sql()).await.unwrap();

    batch.id
}

async fn insert_allocation(db: &SqlitePool, order_line_id: String, batch_id: String) -> String {
    let allocation = Allocation {
        order_line_id,
        batch_id,
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    create(db, &allocation.insert_sql()).await.unwrap();

    allocation.id
}

#[tokio::test]
async fn test_query_to_json_try_decode() {
    let db = in_memory_db().await;
    start_mappers(&db).await;
    db.execute(
        "INSERT INTO test_table (name, age, active) VALUES
            ('Alice', 30, 1),
            ('Bob', NULL, 0)",
    )
    .await
    .unwrap();
    let results = read_to_json(&db, "SELECT * FROM test_table").await.unwrap();
    let expected = vec![
        json!({
            "id": 1,
            "name": "Alice",
            "age": 30,
            "active": 1,
        }),
        json!({
            "id": 2,
            "name": "Bob",
            "age": null,
            "active": 0,
        }),
    ];
    assert_eq!(results, expected);
}
