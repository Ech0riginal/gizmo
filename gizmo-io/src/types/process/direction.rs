use crate::*;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Direction {
    Out,
    In,
    From,
    To,
}

obj!(Direction);
tag!(Direction);
