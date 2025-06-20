use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Cardinality {
    List,
    Set,
    Single,
}

obj!(Cardinality);
tag!(Cardinality);
