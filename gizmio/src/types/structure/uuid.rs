use crate::*;

pub type Uuid = ::uuid::Uuid;

obj!(Uuid);

impl<D: Dialect> Tag_<D> for Uuid {
    const tag: &'static str = "g:UUID";
}
