use std::fmt::Formatter;

#[derive(PartialEq, Clone, Eq, Hash)]
pub enum T {
    Id,
    Key,
    Label,
    Value,
}

impl std::fmt::Debug for T {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            T::Id => write!(f, ".Id"),
            T::Key => write!(f, ".Key"),
            T::Label => write!(f, ".Label"),
            T::Value => write!(f, ".Value"),
        }
    }
}
