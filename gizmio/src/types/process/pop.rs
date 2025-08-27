use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Pop {
    All,
    First,
    Last,
    Mixed,
}

obj!(Pop);
tag!(Pop);
string_reprs! {
    Pop,
    ALL -> "all",
    FIRST -> "first",
    LAST -> "last",
    MIXED -> "mixed",
}

impl std::fmt::Display for Pop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Pop::All => write!(f, "{}", Pop::ALL),
            Pop::First => write!(f, "{}", Pop::FIRST),
            Pop::Last => write!(f, "{}", Pop::LAST),
            Pop::Mixed => write!(f, "{}", Pop::MIXED),
        }
    }
}
