use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Property {
    pub(crate) key: String,
    pub(crate) value: Box<GValue>,
    pub(crate) element: Box<GValue>,
}

obj!(Property);
tag!(Property);

impl Property {
    pub fn new<K, V, E>(key: K, value: V, element: E) -> Property
    where
        K: Into<String>,
        V: Into<GValue>,
        E: Into<GValue>,
    {
        Property {
            key: key.into(),
            value: Box::new(value.into()),
            element: Box::new(element.into()), // TODO
        }
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub fn label(&self) -> &String {
        &self.key
    }
}
