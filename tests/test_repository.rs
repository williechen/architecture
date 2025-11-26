use architecture::{configures::AppConfig, entities::batches::Batch};
use rbatis::rbdc::DateTime;

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
