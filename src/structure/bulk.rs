use crate::structure::{GKey, GValue};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct BulkSet {
    pub map: HashMap<GKey, GValue>,
    pub occurrences: usize,
}
