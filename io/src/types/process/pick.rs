use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Pick {
    inner: String,
}

obj!(Pick);
tag!(Pick);
