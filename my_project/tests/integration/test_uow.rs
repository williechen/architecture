use architecture::chapter1;
use architecture::configures;
use architecture::entities::batches;
use architecture::entities::products;
use architecture::repositories::read;
use architecture::repositories::read_one;
use architecture::repositories::update;
use sqlx::SqliteConnection;

fn random_suffix() -> String {
    let s = xid::new().to_string();
    let s_ref = s.as_str();

    let sid = s_ref.chars().rev().take(6).collect::<String>();
    println!("Generated random suffix: {}", sid);
    sid
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

async fn insert_batch(
    db: &sqlx::SqlitePool,
    batch_ref: &str,
    sku: &str,
    qty: i32,
    eta: Option<&str>,
    version: i32,
) {
    let product_ent = format!(
        "INSERT INTO product (sku, version_number) VALUES ('{}', {})",
        sku, version
    );

    sqlx::query(&product_ent).execute(db).await.unwrap();

    let eta_val = match eta {
        Some(date_str) => format!("'{}'", date_str),
        None => "NULL".to_string(),
    };

    let batch_ent = format!(
        "INSERT INTO batch (reference, sku, qty, eta) VALUES ('{}', '{}', {}, {})",
        batch_ref, sku, qty, eta_val
    );

    sqlx::query(&batch_ent).execute(db).await.unwrap();
}

async fn try_to_allocate(
    db: &sqlx::SqlitePool,
    order_id: &str,
    sku: &str,
    barrier: std::sync::Arc<tokio::sync::Barrier>,
) {
    let mut tx = db.begin().await.unwrap();

    let order = chapter1::OrderLine {
        order_id: order_id.to_string(),
        sku: sku.to_string(),
        qty: 10,
    };

    let where_clause_string = format!("sku = '{}'", sku);
    let where_clause = Some(where_clause_string.as_str());

    let product_ent = read_one::<&mut SqliteConnection, products::Product>(
        &mut *tx,
        &products::Product::select_sql(where_clause),
    )
    .await
    .unwrap();

    barrier.wait().await;

    if let Some(ent) = product_ent {
        let batche_ents = read::<&mut SqliteConnection, batches::Batch>(
            &mut *tx,
            &batches::Batch::select_sql(where_clause),
        )
        .await
        .unwrap();

        let batches = batche_ents
            .into_iter()
            .map(|b| b.build())
            .collect::<Vec<chapter1::Batch>>();

        let res = ent.build(batches).allocate(&order);
        match res {
            Ok(batches_ref) => {
                let batch_ref = batches_ref.unwrap();

                let product_res = update::<&mut SqliteConnection>(
                    &mut *tx,
                    &format!(
                        "UPDATE product SET version_number = {} WHERE sku = '{}' AND version_number = {}",
                        batch_ref.1, sku, ent.version_number
                    ),
                )
                .await;
                match product_res {
                    Ok(result) => {
                        update::<&mut SqliteConnection>(
                            &mut *tx,
                            &format!(
                                "UPDATE batch SET qty = qty - 10 WHERE reference = '{}'",
                                batch_ref.0
                            ),
                        )
                        .await
                        .unwrap();

                        println!("Order id {} allocated to batch {}", order_id, batch_ref.0);

                        if result.rows_affected() == 1 {
                            tx.commit().await.unwrap();
                        } else {
                            tx.rollback().await.unwrap();
                        }
                    }
                    Err(_) => {
                        println!(
                            "Failed to update product version number for order id {}",
                            order_id
                        );
                        tx.rollback().await.unwrap();
                    }
                }
            }
            Err(_) => {
                tx.rollback().await.unwrap();
            }
        }
    } else {
        tx.rollback().await.unwrap();
    }
}

#[tokio::test]
#[ignore]
async fn test_concurrent_updates_to_version_are_not_allowed() {
    let db = configures::AppConfig::load()
        .database
        .get_connection()
        .await;

    let sku = random_sku("");
    let batch = random_batch_ref("");
    insert_batch(&db, &batch, &sku, 100, None, 1).await;

    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(2));

    let handle1 = tokio::spawn({
        let db = db.clone();
        let barrier = barrier.clone();
        let sku = sku.clone();
        let order1 = random_order_id("1");
        async move {
            try_to_allocate(&db, &order1, &sku, barrier).await;
        }
    });

    let handle2 = tokio::spawn({
        let db = db.clone();
        let barrier = barrier.clone();
        let sku = sku.clone();
        let order2 = random_order_id("2");
        async move {
            try_to_allocate(&db, &order2, &sku, barrier).await;
        }
    });

    let (res1, res2) = tokio::join!(handle1, handle2);

    if let Err(e) = res1 {
        panic!("Task 1 failed: {:?}", e);
    }
    if let Err(e) = res2 {
        panic!("Task 2 failed: {:?}", e);
    }

    let version_number: Option<(i32,)> = read_one::<&sqlx::SqlitePool, (i32,)>(
        &db,
        &format!(
            "SELECT MAX(version_number) FROM product WHERE sku = '{}'",
            sku
        ),
    )
    .await
    .unwrap();

    assert_eq!(version_number.unwrap().0, 2);
}
