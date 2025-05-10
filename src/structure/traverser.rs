use crate::structure::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Traverser {
    bulk: i64,
    value: Box<GValue>,
}

impl Traverser {
    pub fn new(bulk: i64, value: GValue) -> Traverser {
        Traverser {
            bulk,
            value: Box::new(value),
        }
    }
}
