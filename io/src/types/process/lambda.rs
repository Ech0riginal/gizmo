use crate::{List, obj};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Lambda {
    pub(crate) script: String,
    pub(crate) language: String,
    pub(crate) arguments: List<String>,
}

obj!(Lambda);
