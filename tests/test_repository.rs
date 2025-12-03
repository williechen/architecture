use architecture::{
    chapter1::model,
    configures::AppConfig,
    entities::{allocations::Allocation, batches::Batch, order_lines::OrderLine},
};
use chrono::Local;
use rbatis::RBatis;

async fn in_memory_db() -> RBatis {
    let conn_str = "sqlite://:memory:";
    let rbatis = RBatis::new();
    rbatis.link(SqliteDriver {}, &conn_str).await.unwrap();
    rbatis
}

async fn start_mappers(db: &RBatis) {
    let mapper = &table_sync::SqliteTableMapper {} as &dyn table_sync::ColumnMapper;

    RBatis::sync(
        db,
        mapper, // Assuming UamUser implements ColumnMapper
        &batches::Batch::default(),
        "batches",
    )
    .await
    .unwrap();

    RBatis::sync(
        db,
        mapper, // Assuming UamUser implements ColumnMapper
        &order_lines::OrderLine::default(),
        "order_lines",
    )
    .await
    .unwrap();

    RBatis::sync(
        db,
        mapper, // Assuming UamUser implements ColumnMapper
        &allocations::Allocation::default(),
        "allocations",
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
        purchased_quantity: 100,
        eta: None,
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    Batch::insert(&db, &batch).await.unwrap();

    let fetched_batch = Batch::select_all(&db).await.unwrap();

    assert_eq!(fetched_batch.len(), 1);
    assert_eq!(fetched_batch[0].reference, "batch1");
    assert_eq!(fetched_batch[0].sku, "RUSTY-SOAPDISH");
    assert_eq!(fetched_batch[0].purchased_quantity, 100);
    assert_eq!(fetched_batch[0].eta, None);
}

#[tokio::test]
async fn test_repository_can_retrieve_a_batch_with_allocations() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    let order_line_id = insert_order_line(&db).await;
    let batch_id = insert_batch(&db, "batch1".to_string()).await;
    insert_batch(&db, "batch2".to_string()).await;
    insert_allocation(&db, order_line_id.clone(), batch_id.clone()).await;

    let fetched_batch = Batch::select_by_reference(&db, "batch1").await.unwrap();
    let fetched_order_line = OrderLine::select_by_order_id(&db, "order1").await.unwrap();

    let expected = model::Batch::new("batch1", "GENERIC-SOFA", 100, None);
    let expected_order = model::OrderLine {
        order_id: "order1".to_string(),
        sku: "GENERIC-SOFA".to_string(),
        qty: 12,
    };

    assert_eq!(fetched_batch[0].reference, expected.reference);
    assert_eq!(fetched_batch[0].sku, expected.sku);
    assert_eq!(
        fetched_batch[0].purchased_quantity,
        expected.allocated_quantity()
    );

    assert_eq!(fetched_order_line[0].sku, expected_order.sku);
    assert_eq!(fetched_order_line[0].order_id, expected_order.order_id);
    assert_eq!(fetched_order_line[0].qty, expected_order.qty);
}

async fn insert_order_line(db: &RBatis) -> String {
    let order_line = OrderLine {
        order_id: "order1".to_string(),
        sku: "GENERIC-SOFA".to_string(),
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
        qty: 12,
    };

    OrderLine::insert(db, &order_line).await.unwrap();

    order_line.id
}

async fn insert_batch(db: &RBatis, batch_id: String) -> String {
    let batch = Batch {
        reference: batch_id,
        sku: "GENERIC-SOFA".to_string(),
        purchased_quantity: 100,
        eta: None,
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    Batch::insert(db, &batch).await.unwrap();

    batch.id
}

async fn insert_allocation(db: &RBatis, order_line_id: String, batch_id: String) -> String {
    let allocation = Allocation {
        order_line_id,
        batch_id,
        id: xid::new().to_string(),
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    Allocation::insert(db, &allocation).await.unwrap();

    allocation.id
}
