use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Path {
    pub(crate) labels: Box<GValue>,
    pub(crate) objects: Box<GValue>,
}

obj!(Path);
tag!(Path);
