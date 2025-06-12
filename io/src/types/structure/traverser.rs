use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Traverser {
    pub(crate) bulk: i64,
    pub(crate) value: Box<GValue>,
}

impl Traverser {
    pub fn new(bulk: i64, value: GValue) -> Traverser {
        Traverser {
            bulk,
            value: Box::new(value),
        }
    }
}

impl Object for Traverser {
    const name: &'static str = "Traverser";
}
