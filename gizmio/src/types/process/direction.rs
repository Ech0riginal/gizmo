use crate::*;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Direction {
    Out,
    In,
    From,
    To,
}

string_reprs! {
    Direction,
    OUT -> "OUT",
    IN -> "IN",
}

obj!(Direction);
tag!(Direction);
