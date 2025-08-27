use crate::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Scope {
    Global,
    Local,
}

obj!(Scope);
tag!(Scope);
string_reprs! {
    Scope,
    GLOBAL -> "global",
    LOCAL -> "local",
}

impl From<()> for Scope {
    fn from(_val: ()) -> Self {
        Scope::Global
    }
}
