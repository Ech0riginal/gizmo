#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Order {
    Asc,
    Desc,
    Shuffle,
}
