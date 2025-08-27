use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Pick {
    Any,
    None,
}

obj!(Pick);
tag!(Pick);
string_reprs! {
    Pick,
    ANY -> "any",
    NONE -> "none",
}
