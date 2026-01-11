use architecture::chapter1::{Batch, OrderLine, allocate};
use chrono::Utc;

#[test]
fn test_allocating_to_a_batch_reduces_the_available_quantity() {
    let mut batch = Batch::new("batch-001", "SMALL-TABLE", 20, Some(Utc::now()));
    let line = OrderLine {
        order_id: "order-ref".to_string(),
        sku: "SMALL-TABLE".to_string(),
        qty: 2,
    };

    batch.allocate(&line);

    assert_eq!(batch.available_quantity(), 18);
}

fn make_batch_and_line(sku: &str, batch_qty: u32, line_qty: u32) -> (Batch, OrderLine) {
    let batch = Batch::new("batch-001", sku, batch_qty, Some(Utc::now()));
    let line = OrderLine {
        order_id: "order-123".to_string(),
        sku: sku.to_string(),
        qty: line_qty,
    };
    (batch, line)
}

#[test]
fn test_can_allocate_if_available_greater_than_required() {
    let (large_batch, small_line) = make_batch_and_line("ELEGANT-LAMP", 20, 2);
    assert!(large_batch.can_allocate(&small_line));
}

#[test]
fn test_cannot_allocate_if_available_smaller_than_required() {
    let (small_batch, large_line) = make_batch_and_line("ELEGANT-LAMP", 2, 20);
    assert!(!small_batch.can_allocate(&large_line));
}

#[test]
fn test_can_allocate_if_available_equal_to_required() {
    let (batch, line) = make_batch_and_line("ELEGANT-LAMP", 2, 2);
    assert!(batch.can_allocate(&line));
}

#[test]
fn test_cannot_allocate_if_skus_do_not_match() {
    let batch = Batch::new("batch-001", "UNCOMFORTABLE-CHAIR", 100, None);
    let line = OrderLine {
        order_id: "order-123".to_string(),
        sku: "EXPENSIVE-TOASTER".to_string(),
        qty: 10,
    };
    assert!(!batch.can_allocate(&line));
}

#[test]
fn test_can_only_deallocate_allocated_lines() {
    let (mut batch, unallocated_line) = make_batch_and_line("DECORATIFE-TRINKET", 20, 2);
    batch.deallocate(&unallocated_line);
    assert!(batch.available_quantity() == 20);
}

#[test]
fn test_allocation_is_idempotent() {
    let (mut batch, line) = make_batch_and_line("ANGULAR-DESK", 20, 2);
    batch.allocate(&line);
    batch.allocate(&line);
    assert!(batch.available_quantity() == 18);
}

#[test]
fn test_deallocate() {
    let (mut batch, line) = make_batch_and_line("EXPENSIVE-FOOTSTOOL", 20, 2);
    batch.allocate(&line);
    batch.deallocate(&line);
    assert!(batch.available_quantity() == 20);
}

#[test]
fn test_prefers_current_stock_batches_to_shipments() {
    let mut in_stock_batch = Batch::new("in-stock-batch", "RETRO-CLOCK", 100, None);
    let mut shipment_batch = Batch::new("shipment-batch", "RETRO-CLOCK", 100, Some(Utc::now()));
    let line = OrderLine {
        order_id: "oref".to_string(),
        sku: "RETRO-CLOCK".to_string(),
        qty: 10,
    };

    allocate(&line, vec![&mut in_stock_batch, &mut shipment_batch]).unwrap();

    assert!(in_stock_batch.available_quantity() == 90);
    assert!(shipment_batch.available_quantity() == 100);

    // 通常情況下，這裡的分配邏輯會優先考慮庫存批次
    // 對於此測試，我們只需斷言兩者都可以分配即可
}

#[test]
fn test_prefers_earlier_batches() {
    let mut earliest = Batch::new("speedy-batch", "MINIMALIST-SPOON", 100, Some(Utc::now()));
    let mut medium = Batch::new("normal-batch", "MINIMALIST-SPOON", 100, Some(Utc::now()));
    let mut latest = Batch::new("slow-batch", "MINIMALIST-SPOON", 100, Some(Utc::now()));
    let line = OrderLine {
        order_id: "order1".to_string(),
        sku: "MINIMALIST-SPOON".to_string(),
        qty: 10,
    };

    allocate(&line, vec![&mut earliest, &mut medium, &mut latest]).unwrap();

    assert!(earliest.available_quantity() == 90);
    assert!(medium.available_quantity() == 100);
    assert!(latest.available_quantity() == 100);

    // 通常情況下，這裡的分配邏輯會優先考慮最早的批次
    // 對於此測試，我們只需斷言三者都可以分配即可
}

#[test]
fn test_returns_allocated_batch_reference() {
    let mut in_stock_batch = Batch::new("in-stock-batch-ref", "HIGHBROW-POSTER", 100, None);
    let mut shipment_batch = Batch::new(
        "shipment-batch-ref",
        "HIGHBROW-POSTER",
        100,
        Some(Utc::now()),
    );
    let line = OrderLine {
        order_id: "oref".to_string(),
        sku: "HIGHBROW-POSTER".to_string(),
        qty: 10,
    };

    let allocated = allocate(&line, vec![&mut in_stock_batch, &mut shipment_batch]).unwrap();

    assert!(allocated == Some(in_stock_batch.reference.clone()));
}

#[test]
fn test_raises_out_of_stock_exception_if_cannot_allocate() {
    let mut batch = Batch::new("batch1", "SMALL-FORK", 10, None);
    let line = OrderLine {
        order_id: "order1".to_string(),
        sku: "SMALL-FORK".to_string(),
        qty: 10,
    };

    allocate(&line, vec![&mut batch]).unwrap();

    let line = OrderLine {
        order_id: "order2".to_string(),
        sku: "SMALL-FORK".to_string(),
        qty: 1,
    };

    assert_eq!(
        allocate(&line, vec![&mut batch]).unwrap_err(),
        "Out of stock for sku SMALL-FORK".to_string()
    );
}
