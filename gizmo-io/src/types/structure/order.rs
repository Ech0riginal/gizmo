use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Order {
    Asc,
    Desc,
    Shuffle,
}

obj!(Order);
tag!(Order);
string_reprs! {
    Order,
    ASC -> "asc",
    DESC -> "desc",
    SHUFFLE -> "shuffle",
}
