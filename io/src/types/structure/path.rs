use crate::{GValue, Object};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Path {
    pub(crate) labels: Box<GValue>,
    pub(crate) objects: Box<GValue>,
}

impl Object for Path {
    const name: &'static str = "Path";
}
