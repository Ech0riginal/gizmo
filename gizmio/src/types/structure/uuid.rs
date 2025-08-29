use crate::*;

pub type Uuid = ::uuid::Uuid;

obj!(Uuid);

impl<D: Dialect> AST<D> for Uuid {
    const tag: &'static str = "g:UUID";
}
