#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Scope {
    Global,
    Local,
}

impl Into<Scope> for () {
    fn into(self) -> Scope {
        Scope::Global
    }
}
