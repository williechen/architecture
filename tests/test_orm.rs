use architecture::entities::order_lines::OrderLine;

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
async fn test_orderline_mapper_can_load_lines() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    db.query_decode(
        r"
        INSERT INTO order_lines (id, order_id, sku, qty, created_at, updated_at)
        VALUES ('1', 'order1', 'RED-CHAIR', 12, datetime('now'), datetime('now')),
               ('2', 'order2', 'RED-TABLE', 13, datetime('now'), datetime('now')),
               ('3', 'order3', 'BLUE-LIPSTICK', 14, datetime('now'), datetime('now'))
    ",
        vec![],
    )
    .await
    .unwrap();

    let expected = vec![
        OrderLine {
            id: "1".to_string(),
            order_id: "order1".to_string(),
            sku: "RED-CHAIR".to_string(),
            qty: 12,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
        OrderLine {
            id: "2".to_string(),
            order_id: "order2".to_string(),
            sku: "RED-TABLE".to_string(),
            qty: 13,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
        OrderLine {
            id: "3".to_string(),
            order_id: "order3".to_string(),
            sku: "BLUE-LIPSTICK".to_string(),
            qty: 14,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
    ];

    let lines: Vec<OrderLine> = OrderLine::select_all(&db).await.unwrap();

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
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    OrderLine::insert(&db, &new_line).await.unwrap();

    let fetched_line: Option<OrderLine> = OrderLine::select_all(&db).await.unwrap();

    assert!(fetched_line == new_line);
}

#[tokio::test]
async fn test_retrieving_batches() {
    let db = in_memory_db().await;
    start_mappers(&db).await;

    // Insert test data into batches table
    db.query_decode(
        r"
        INSERT INTO batches (id, reference, sku, purchased_quantity, eta, created_at, updated_at)
        VALUES ('1', 'batch1', 'sku1', 100, NULL, datetime('now'), datetime('now')),
               ('2', 'batch2', 'sku2', 200, '2011-04-11', datetime('now'), datetime('now'))
    ",
        vec![],
    )
    .await
    .unwrap();

    // Retrieve batches
    let batches: Vec<batches::Batch> = batches::Batch::select_all(&db).await.unwrap();

    let expected = vec![
        batches::Batch {
            id: "1".to_string(),
            reference: "batch1".to_string(),
            sku: "sku1".to_string(),
            purchased_quantity: 100,
            eta: None,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
        batches::Batch {
            id: "2".to_string(),
            reference: "batch2".to_string(),
            sku: "sku2".to_string(),
            purchased_quantity: 200,
            eta: Some(chrono::NaiveDate::from_ymd(2011, 4, 11)),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
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
        purchased_quantity: 100,
        eta: None,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    batches::Batch::insert(&db, &new_batch).await.unwrap();

    let fetched_batch: Vec<batches::Batch> = batches::Batch::select_all(&db).await.unwrap();

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
        purchased_quantity: 100,
        eta: None,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let order_line = OrderLine {
        id: "1".to_string(),
        order_id: "order1".to_string(),
        sku: "sku1".to_string(),
        qty: 10,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let new_allocation = allocations::Allocation {
        id: "1".to_string(),
        order_line_id: order_line.id.clone(),
        batch_id: batch.id.clone(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    allocations::Allocation::insert(&db, &new_allocation)
        .await
        .unwrap();

    let fetched_allocation: Vec<allocations::Allocation> =
        allocations::Allocation::select_all(&db).await.unwrap();

    assert!(fetched_allocation == vec![new_allocation]);
}

#[tokio::test]
async fn test_retrieving_allocations() {
    let db = in_memory_db().await;
    start_mappers(&db).await;
    // Insert test data into allocations table
    db.query_decode(
        r"
        INSERT INTO allocations (id, order_line_id, batch_id, created_at, updated_at)
        VALUES ('1', '1', '1', datetime('now'), datetime('now')),
               ('2', '2', '2', datetime('now'), datetime('now'))
    ",
        vec![],
    )
    .await
    .unwrap();

    // Retrieve allocations
    let allocations: Vec<allocations::Allocation> =
        allocations::Allocation::select_all(&db).await.unwrap();
    let expected = vec![
        allocations::Allocation {
            id: "1".to_string(),
            order_line_id: "1".to_string(),
            batch_id: "1".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
        allocations::Allocation {
            id: "2".to_string(),
            order_line_id: "2".to_string(),
            batch_id: "2".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
    ];

    assert!(allocations == expected);
}
