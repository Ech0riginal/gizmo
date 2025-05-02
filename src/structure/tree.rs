use crate::structure::GValue;

#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub(crate) branches: Vec<Branch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Branch {
    pub(crate) key: Box<GValue>,
    pub(crate) value: Box<GValue>,
}
