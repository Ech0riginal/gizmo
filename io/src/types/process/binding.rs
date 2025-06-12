use crate::GValue;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Binding {
    pub(crate) key: String,
    pub(crate) value: Box<GValue>,
}

crate::obj!(Binding);
