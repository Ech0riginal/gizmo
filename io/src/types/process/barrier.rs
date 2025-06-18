use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Barrier {
    NormSack,
}

obj!(Barrier);
tag!(Barrier);
