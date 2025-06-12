#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Scope {
    Global,
    Local,
}

impl From<()> for Scope {
    fn from(_val: ()) -> Self {
        Scope::Global
    }
}
