use crate::{Edge, List, Object, Vertex};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TinkerGraph {
    pub(crate) vertices: List<Vertex>,
    pub(crate) edges: List<Edge>,
}

impl Object for TinkerGraph {
    const name: &'static str = "TinkerGraph";
}
impl Hash for TinkerGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in self.vertices.iter() {
            v.hash(state);
        }
        for e in self.edges.iter() {
            e.hash(state);
        }
    }
}
