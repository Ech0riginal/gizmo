use crate::Object;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Pop {
    All,
    First,
    Last,
    Mixed,
}

impl Object for Pop {
    const name: &'static str = "Pop";
}

impl Display for Pop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Pop::All => write!(f, "all"),
            Pop::First => write!(f, "first"),
            Pop::Last => write!(f, "last"),
            Pop::Mixed => write!(f, "mixed"),
        }
    }
}
