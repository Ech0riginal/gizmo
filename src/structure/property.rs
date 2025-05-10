use crate::{GremlinError, GremlinResult};
// use crate::conversion::{BorrowFromGValue, FromGValue};
use crate::structure::*;

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

    pub fn label(&self) -> &String {
        &self.key
    }
}

trait GRefs: Sized {
    fn value(self) -> GValue;
    fn value_ref(&self) -> &GValue;

    fn take<T>(self) -> GremlinResult<T>
    where
        T: TryFrom<GValue, Error = GremlinError>,
    {
        T::try_from(self.value())
    }

    fn get<'a, T>(&'a self) -> GremlinResult<T>
    where
        T: TryFrom<&'a GValue, Error = GremlinError>,
    {
        T::try_from(self.value_ref())
    }
}
