use crate::{GValue, List, Object, obj, tag};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tree {
    pub(crate) branches: List<Branch>,
}

obj!(Tree);
tag!(Tree);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Branch {
    pub(crate) key: Box<GValue>,
    pub(crate) value: Box<GValue>,
}

obj!(Branch);
tag!(Branch);
