use std::{collections::HashSet, error::Error};

use crate::chapter1::Batch;

pub struct Fake {
    _batches: HashSet<Batch>,
}

impl Fake {
    pub fn read(&self) -> Result<Vec<Batch>, Box<dyn Error>> {
        Ok(self._batches.iter().cloned().collect())
    }

    pub fn read_one(&self, reference: String) -> Result<Option<Batch>, Box<dyn Error>> {
        Ok(self
            ._batches
            .iter()
            .cloned()
            .find(|b| b.reference == reference))
    }

    pub fn create(&mut self, batch: Batch) -> Result<i32, Box<dyn Error>> {
        self._batches.insert(batch);
        Ok(self._batches.len() as i32)
    }

    pub fn update(&mut self, batch: Batch) -> Result<i32, Box<dyn Error>> {
        self._batches.replace(batch);
        Ok(self._batches.len() as i32)
    }

    pub fn delete(&mut self, reference: String) -> Result<i32, Box<dyn Error>> {
        self._batches.retain(|b| b.reference != reference);
        Ok(self._batches.len() as i32)
    }
}
