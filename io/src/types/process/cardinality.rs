use crate::Object;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Cardinality {
    List,
    Set,
    Single,
}

impl Object for Cardinality {
    const name: &'static str = "Cardinality";
}
