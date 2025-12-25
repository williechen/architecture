use std::collections::HashSet;

use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OrderLine {
    pub order_id: String,
    pub sku: String,
    pub qty: u32,
}

#[derive(Debug, Clone, Eq)]
pub struct Batch {
    pub reference: String,
    pub sku: String,
    pub eta: Option<NaiveDateTime>,
    _purchased_quantity: u32,
    _allocated_lines: HashSet<OrderLine>,
}

impl Batch {
    pub fn new(reference: &str, sku: &str, qty: u32, eta: Option<NaiveDateTime>) -> Self {
        Batch {
            reference: reference.to_string(),
            sku: sku.to_string(),
            eta: eta,
            _purchased_quantity: qty,
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

    pub fn allocated_quantity(&self) -> u32 {
        self._allocated_lines.iter().map(|line| line.qty).sum()
    }

    pub fn available_quantity(&self) -> u32 {
        self._purchased_quantity - self.allocated_quantity()
    }

    pub fn can_allocate(&self, line: &OrderLine) -> bool {
        self.sku == line.sku && self.available_quantity() >= line.qty
    }
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        self.reference == other.reference
    }
}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eta.is_none() && other.eta.is_none() {
            Some(std::cmp::Ordering::Equal)
        } else if self.eta.is_none() {
            Some(std::cmp::Ordering::Less)
        } else if other.eta.is_none() {
            Some(std::cmp::Ordering::Greater)
        } else {
            self.eta.unwrap().partial_cmp(&other.eta.unwrap())
        }
    }
}

impl std::hash::Hash for Batch {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
    }
}

pub fn allocate(
    line: &OrderLine,
    batches: Vec<&mut Batch>,
) -> Result<Option<String>, &'static str> {
    let mut batch_vec: Vec<&mut Batch> = batches
        .into_iter()
        .filter(|b| b.can_allocate(line))
        .collect();
    if !batch_vec.is_empty() {
        batch_vec[0].allocate(line);
        return Ok(Some(batch_vec[0].reference.clone()));
    } else {
        return Err("Out of stock");
    }
}
