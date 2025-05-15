use crate::structure::{Edge, Vertex};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TinkerGraph {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) edges: Vec<Edge>,
}
