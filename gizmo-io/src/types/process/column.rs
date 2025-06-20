use crate::*;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Column {
    Keys,
    Values,
}

obj!(Column);
tag!(Column);
