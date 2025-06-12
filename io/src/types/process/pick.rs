#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Pick {
    inner: String,
}

crate::obj!(Pick);
