use crate::Object;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Scope {
    Global,
    Local,
}

impl Object for Scope {
    const name: &'static str = "Scope";
}

impl From<()> for Scope {
    fn from(_val: ()) -> Self {
        Scope::Global
    }
}
