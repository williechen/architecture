use std::collections::HashSet;

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    pub orderid: String,
    pub sku: String,
    pub qty: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    pub reference: String,
    pub sku: String,
    pub eta: Option<NaiveDate>,
    pub available_quantity: u32,
    _allocated_lines: HashSet<OrderLine>,
}

impl Batch {
    pub fn new(reference: &str, sku: &str, qty: u32, eta: Option<NaiveDate>) -> Self {
        Batch {
            reference: reference.to_string(),
            sku: sku.to_string(),
            eta: eta,
            available_quantity: qty,
            _allocated_lines: HashSet::new(),
        }
    }

    pub fn allocate(&mut self, line: &OrderLine) {
        if self.can_allocate(line) {
            self._allocated_lines.insert(line.clone());
        }
    }

    pub fn deallocate(&mut self, line: &OrderLine) {
        for allocated_line in self._allocated_lines.iter() {
            if allocated_line == line {
                self._allocated_lines.remove(line);
                break;
            }
        }
    }

    pub fn can_allocate(&self, line: &OrderLine) -> bool {
        self.sku == line.sku && self.available_quantity >= line.qty
    }
}
