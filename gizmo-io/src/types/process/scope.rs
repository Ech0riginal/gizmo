use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Scope {
    Global,
    Local,
}

obj!(Scope);
tag!(Scope);

impl From<()> for Scope {
    fn from(_val: ()) -> Self {
        Scope::Global
    }
}
