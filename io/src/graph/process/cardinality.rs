#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Cardinality {
    List,
    Set,
    Single,
}
