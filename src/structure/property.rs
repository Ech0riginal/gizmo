use crate::prelude::{GValue, GremlinResult};

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub(crate) key: String,
    pub(crate) value: Box<GValue>,
    pub(crate) element: Box<GValue>,
}

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

    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: From<GValue>,
    {
        T::from_gvalue(*self.value)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFrom<GValue>,
    {
        T::from_gvalue(&self.value)
    }

    pub fn label(&self) -> &String {
        &self.key
    }
}
