use architecture::chapters::model::{Batch, OrderLine};
use chrono::Local;

#[test]
fn test_allocating_to_a_batch_reduces_the_available_quantity() {
    let mut batch = Batch::new(
        "batch-001",
        "SMALL-TABLE",
        20,
        Some(Local::now().date_naive()),
    );
    let line = OrderLine {
        orderid: "order-123".to_string(),
        sku: "SMALL-TABLE".to_string(),
        qty: 2,
    };

    batch.allocate(&line);

    assert_eq!(batch.available_quantity(), 18);
}

fn make_batch_and_line(sku: &str, batch_qty: u32, line_qty: u32) -> (Batch, OrderLine) {
    let batch = Batch::new("batch-001", sku, batch_qty, Some(Local::now().date_naive()));
    let line = OrderLine {
        orderid: "order-123".to_string(),
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
        orderid: "order-123".to_string(),
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
