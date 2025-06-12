use crate::{GValue, Map, Object};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BulkSet {
    pub map: Map<GValue, GValue>,
    pub occurrences: usize,
}

impl Object for BulkSet {
    const name: &'static str = "BulkSet";
}
impl Eq for BulkSet {}
impl Hash for BulkSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.map.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}
