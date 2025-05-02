use crate::structure::{Edge, Vertex};

#[derive(Debug, Clone, PartialEq)]
pub struct TinkerGraph {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) edges: Vec<Edge>,
}
