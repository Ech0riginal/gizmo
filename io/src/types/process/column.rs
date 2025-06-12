use crate::Object;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Column {
    Keys,
    Values,
}

impl Object for Column {
    const name: &'static str = "Column";
}
