use std::convert::Infallible;

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Traverser {
    pub bulk: Long,
    pub value: Box<GValue>,
}

obj!(Traverser);
tag!(Traverser);

impl Traverser {
    pub fn new(bulk: Long, value: GValue) -> Traverser {
        Traverser {
            bulk,
            value: Box::new(value),
        }
    }

    pub fn take<T>(self) -> Result<T, Error>
    where
        T: TryFrom<GValue, Error = Infallible>,
    {
        T::try_from(*self.value).map_err(|_| Error::Infallible)
    }
}
