//! Just here so we don't get fined by the compiler

use crate::GValue;

pub trait GValued {
    fn gvalue(self) -> GValue;
}

impl<T> GValued for T
where
    T: Into<GValue>,
{
    fn gvalue(self) -> GValue {
        self.into()
    }
}
