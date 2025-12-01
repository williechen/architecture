use std::collections::HashSet;

use crate::chapter1::model;

pub struct Fake {
    _batches: HashSet<model::Batch>,
}

impl repository for Fake {
    
}
