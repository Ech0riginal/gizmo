use crate::*;

#[derive(PartialEq, Clone, Eq, Hash)]
pub enum T {
    Id,
    Key,
    Label,
    Value,
}

obj!(T);
tag!(T);
string_reprs! {
    T,
    ID -> "id",
    KEY -> "key",
    LABEL -> "label",
    VALUE -> "value",
}

impl std::fmt::Debug for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            T::Id => write!(f, ".Id"),
            T::Key => write!(f, ".Key"),
            T::Label => write!(f, ".Label"),
            T::Value => write!(f, ".Value"),
        }
    }
}
