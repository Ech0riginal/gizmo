use crate::{GValue, Map2};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BulkSet {
    pub map: Map2<GValue, GValue>,
    pub occurrences: usize,
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
