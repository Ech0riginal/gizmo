use crate::Object;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Order {
    Asc,
    Desc,
    Shuffle,
}

impl Object for Order {
    const name: &'static str = "Order";
}
