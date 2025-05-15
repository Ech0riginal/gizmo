use crate::structure::GValue;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tree {
    pub(crate) branches: Vec<Branch>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Branch {
    pub(crate) key: Box<GValue>,
    pub(crate) value: Box<GValue>,
}
