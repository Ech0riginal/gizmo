use crate::{GValue, List};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tree {
    pub(crate) branches: List<Branch>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Branch {
    pub(crate) key: Box<GValue>,
    pub(crate) value: Box<GValue>,
}
