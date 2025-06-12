use crate::Object;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Direction {
    Out,
    In,
    From,
    To,
}

impl Object for Direction {
    const name: &'static str = "Direction";
}
