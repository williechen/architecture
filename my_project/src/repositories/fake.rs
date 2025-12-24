use std::{collections::HashSet, error::Error};

use crate::entities::batches;

pub struct Fake {
    _batches: HashSet<batches::Batch>,
}

impl Fake {
    pub fn read(&self) -> Result<Vec<batches::Batch>, Box<dyn Error>> {
        Ok(self._batches.iter().cloned().collect())
    }

    pub fn read_one(&self, id: String) -> Result<Option<batches::Batch>, Box<dyn Error>> {
        Ok(self._batches.iter().cloned().find(|b| b.id == id))
    }

    pub fn create(&mut self, batch: batches::Batch) -> Result<i32, Box<dyn Error>> {
        self._batches.insert(batch);
        Ok(self._batches.len() as i32)
    }

    pub fn update(&mut self, batch: batches::Batch) -> Result<i32, Box<dyn Error>> {
        self._batches.replace(batch);
        Ok(self._batches.len() as i32)
    }

    pub fn delete(&mut self, id: String) -> Result<i32, Box<dyn Error>> {
        self._batches.retain(|b| b.id != id);
        Ok(self._batches.len() as i32)
    }
}
