use crate::{Dialect, Edge, List, Tag_, Vertex, obj};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TinkerGraph {
    pub(crate) vertices: List<Vertex>,
    pub(crate) edges: List<Edge>,
}

obj!(TinkerGraph);

impl<D: Dialect> Tag_<D> for TinkerGraph {
    const tag: &'static str = "tinker:graph";
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
