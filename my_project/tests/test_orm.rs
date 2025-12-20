use architecture::entities::{
    allocations, batches,
    order_lines::{self, OrderLine},
};
use architecture::repositories::create;
use architecture::repositories::read;
use architecture::repositories::read_one;
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
async fn test_orderline_mapper_can_load_lines() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    db.execute(
        r"
        INSERT INTO order_lines (id, order_id, sku, qty, created_at, updated_at)
        VALUES ('1', 'order1', 'RED-CHAIR', 12, '2025-12-08T00:00:00', '2025-12-08T00:00:00'),
               ('2', 'order2', 'RED-TABLE', 13, '2025-12-08T00:00:00', '2025-12-08T00:00:00'),
               ('3', 'order3', 'BLUE-LIPSTICK', 14, '2025-12-08T00:00:00', '2025-12-08T00:00:00')
    ",
    )
    .await
    .unwrap();

    let expected = vec![
        OrderLine {
            id: "1".to_string(),
            order_id: "order1".to_string(),
            sku: "RED-CHAIR".to_string(),
            qty: 12,
            created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
        OrderLine {
            id: "2".to_string(),
            order_id: "order2".to_string(),
            sku: "RED-TABLE".to_string(),
            qty: 13,
            created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
        OrderLine {
            id: "3".to_string(),
            order_id: "order3".to_string(),
            sku: "BLUE-LIPSTICK".to_string(),
            qty: 14,
            created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
    ];

    let lines: Vec<OrderLine> = read::<OrderLine>(&db, &OrderLine::select_sql(None))
        .await
        .unwrap();

    assert!(lines.len() == expected.len());
    assert!(lines == expected);
}

#[tokio::test]
async fn test_orderline_mapper_can_save_line() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    let new_line = OrderLine {
        id: "1".to_string(),
        order_id: "order1".to_string(),
        sku: "DECORATIVE-WIDGET".to_string(),
        qty: 12,
        created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    create(&db, &new_line.insert_sql()).await.unwrap();

    let fetched_line = read_one::<OrderLine>(
        &db,
        &OrderLine::select_sql(Some(&OrderLine::where_eq("id", "1"))),
    )
    .await
    .unwrap();

    assert!(fetched_line == Some(new_line));
}

#[tokio::test]
async fn test_retrieving_batches() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    // Insert test data into batches table
    db.execute(
        r"
        INSERT INTO batches (id, reference, sku, qty, eta, created_at, updated_at)
        VALUES ('1', 'batch1', 'sku1', 100, NULL, '2025-12-08T00:00:00', '2025-12-08T00:00:00'),
               ('2', 'batch2', 'sku2', 200, '2011-04-11T00:00:00', '2025-12-08T00:00:00', '2025-12-08T00:00:00')
    ",
    )
    .await
    .unwrap();

    // Retrieve batches
    let batches: Vec<batches::Batch> =
        read::<batches::Batch>(&db, &batches::Batch::select_sql(None))
            .await
            .unwrap();

    let expected = vec![
        batches::Batch {
            id: "1".to_string(),
            reference: "batch1".to_string(),
            sku: "sku1".to_string(),
            qty: 100,
            eta: None,
            created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
        batches::Batch {
            id: "2".to_string(),
            reference: "batch2".to_string(),
            sku: "sku2".to_string(),
            qty: 200,
            eta: Some(
                chrono::NaiveDate::from_ymd_opt(2011, 4, 11)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
            created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        },
    ];

    assert!(batches.len() == expected.len());
    assert!(batches == expected);
}

#[tokio::test]
async fn test_saving_batches() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    let new_batch = batches::Batch {
        id: "1".to_string(),
        reference: "batch1".to_string(),
        sku: "sku1".to_string(),
        qty: 100,
        eta: None,
        created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    create(&db, &new_batch.insert_sql()).await.unwrap();

    let fetched_batch: Vec<batches::Batch> =
        read::<batches::Batch>(&db, &batches::Batch::select_sql(None))
            .await
            .unwrap();

    assert!(fetched_batch == vec![new_batch]);
}

#[tokio::test]
async fn test_saving_allocations() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    let batch = batches::Batch {
        id: "1".to_string(),
        reference: "batch1".to_string(),
        sku: "sku1".to_string(),
        qty: 100,
        eta: None,
        created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    let order_line = OrderLine {
        id: "1".to_string(),
        order_id: "order1".to_string(),
        sku: "sku1".to_string(),
        qty: 10,
        created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    let new_allocation = allocations::Allocation {
        id: "1".to_string(),
        order_line_id: order_line.id.clone(),
        batch_id: batch.id.clone(),
        created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    create(&db, &batch.insert_sql()).await.unwrap();
    create(&db, &order_line.insert_sql()).await.unwrap();
    create(&db, &new_allocation.insert_sql()).await.unwrap();

    let fetched_allocation: Vec<allocations::Allocation> =
        read::<allocations::Allocation>(&db, &allocations::Allocation::select_sql(None))
            .await
            .unwrap();

    assert!(fetched_allocation == vec![new_allocation]);
}

#[tokio::test]
async fn test_retrieving_allocations() {
    let db = in_memory_db().await;
    start_mappers(&db).await;
    // Insert test data into allocations table
    db.execute(
        r"
        INSERT INTO order_lines (id, order_id, sku, qty, created_at, updated_at) VALUES ('1', 'order1', 'sku1', 12, '2025-12-08T00:00:00', '2025-12-08T00:00:00')
    ",
    )
    .await
    .unwrap();

    db.execute(
        r"
        INSERT INTO batches (id, reference, sku, qty, eta, created_at, updated_at) VALUES ('1', 'batch1', 'sku1', 100, NULL, '2025-12-08T00:00:00', '2025-12-08T00:00:00')
    ",
    )
    .await
    .unwrap();

    db.execute(
        r"
        INSERT INTO allocations (id, order_line_id, batch_id, created_at, updated_at)
        VALUES ('1', '1', '1', '2025-12-08T00:00:00', '2025-12-08T00:00:00')
    ",
    )
    .await
    .unwrap();

    // Retrieve allocations
    let allocations =
        read::<allocations::Allocation>(&db, &allocations::Allocation::select_sql(None))
            .await
            .unwrap();

    let id = allocations[0].order_line_id.clone();

    let order_lines = read_one::<order_lines::OrderLine>(
        &db,
        &order_lines::OrderLine::select_sql(Some(&order_lines::OrderLine::where_eq("id", &id))),
    )
    .await
    .unwrap();

    let expected = OrderLine {
        id: "1".to_string(),
        order_id: "order1".to_string(),
        sku: "sku1".to_string(),
        qty: 12,
        created_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        updated_at: chrono::NaiveDate::from_ymd_opt(2025, 12, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    };

    assert!(order_lines == Some(expected));
}
