use crate::structure::{GKey, GValue};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone)]
pub struct BulkSet {
    pub map: HashMap<GKey, GValue>,
    pub occurrences: usize,
}

impl Eq for BulkSet {}
impl Hash for BulkSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in &self.map {
            k.hash(state);
            v.hash(state);
        }
    }
}
