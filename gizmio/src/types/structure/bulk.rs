use std::hash::{Hash, Hasher};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BulkSet {
    pub(crate) map: Map<GValue, GValue>,
    pub(crate) occurrences: usize,
}

obj!(BulkSet);
tag!(BulkSet);

impl Eq for BulkSet {}
impl Hash for BulkSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.map.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}
