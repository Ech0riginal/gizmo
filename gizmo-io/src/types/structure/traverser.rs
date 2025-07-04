use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Traverser {
    pub(crate) bulk: Long,
    pub(crate) value: Box<GValue>,
}

obj!(Traverser);
tag!(Traverser);
