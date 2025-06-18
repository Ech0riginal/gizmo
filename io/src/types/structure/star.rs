use crate::{Dialect, GID, List, Map, Tag_, Vertex, VertexProperty, obj};

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq)]
pub struct StarGraph {
    pub(crate) id: GID,
    pub(crate) label: String,
    pub(crate) properties: Map<String, List<VertexProperty>>,
}

obj!(StarGraph);

impl<D: Dialect> Tag_<D> for StarGraph {
    const tag: &'static str = "star";
}

impl From<&StarGraph> for Vertex {
    fn from(value: &StarGraph) -> Self {
        Self {
            id: value.id.clone(),
            label: value.label.clone(),
            properties: value.properties.clone(),
        }
    }
}

impl From<Vertex> for StarGraph {
    fn from(value: Vertex) -> Self {
        Self {
            id: value.id.clone(),
            label: value.label.clone(),
            properties: value.properties.clone(),
        }
    }
}

impl Eq for StarGraph {}
impl Hash for StarGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.label.hash(state);
    }
}
