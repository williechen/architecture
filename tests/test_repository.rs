use architecture::{
    chapter1::model,
    configures::AppConfig,
    entities::{
        allocations::Allocation,
        batches::Batch,
        order_lines::{self, OrderLine},
    },
};
use rbatis::{RBatis, impl_select, rbdc::DateTime};

#[tokio::test]
async fn test_repository_can_save_a_batch() {
    let db = AppConfig::load().database.get_connection().await;

    let batch = Batch {
        reference: "batch1".to_string(),
        sku: "RUSTY-SOAPDISH".to_string(),
        purchased_quantity: 100,
        eta: None,
        id: xid::new().to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    Batch::insert(&db, &batch).await.unwrap();

    let fetched_batch = Batch::select_all(&db).await.unwrap();

    assert_eq!(fetched_batch.len(), 1);
    assert_eq!(fetched_batch[0].reference, "batch1");
    assert_eq!(fetched_batch[0].sku, "RUSTY-SOAPDISH");
    assert_eq!(fetched_batch[0].purchased_quantity, 100);
    assert_eq!(fetched_batch[0].eta, None);

    let id = rbs::value::Value::from(batch.id.clone());
    Batch::delete_by_map(&db, id).await.unwrap();
}

#[tokio::test]
async fn test_repository_can_retrieve_a_batch_with_allocations() {
    let db = AppConfig::load().database.get_connection().await;

    let order_line_id = insert_order_line(&db).await;
    let batch_id = insert_batch(&db, "batch1".to_string()).await;
    let batch_id1 = insert_batch(&db, "batch2".to_string()).await;
    let allocation_id = insert_allocation(&db, order_line_id.clone(), batch_id.clone()).await;

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

    assert_eq!(fetched_order_line[0].sku, Some(expected_order.sku));
    assert_eq!(
        fetched_order_line[0].order_id,
        Some(expected_order.order_id)
    );

    Batch::delete_by_map(&db, rbs::value::Value::from(batch_id))
        .await
        .unwrap();
    Batch::delete_by_map(&db, rbs::value::Value::from(batch_id1))
        .await
        .unwrap();
    OrderLine::delete_by_map(&db, rbs::value::Value::from(order_line_id))
        .await
        .unwrap();
    Allocation::delete_by_map(&db, rbs::value::Value::from(allocation_id))
        .await
        .unwrap();
}

async fn insert_order_line(db: &RBatis) -> String {
    let order_line = OrderLine {
        order_id: Some("order1".to_string()),
        sku: Some("GENERIC-SOFA".to_string()),
        id: xid::new().to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
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
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    Batch::insert(db, &batch).await.unwrap();

    batch.id
}

async fn insert_allocation(db: &RBatis, order_line_id: String, batch_id: String) -> String {
    let allocation = Allocation {
        order_line_id,
        batch_id,
        id: xid::new().to_string(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };

    Allocation::insert(db, &allocation).await.unwrap();

    allocation.id
}
