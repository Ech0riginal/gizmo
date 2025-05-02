use crate::structure::{GID, Vertex, VertexProperty};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct StarGraph {
    pub(crate) id: GID,
    pub(crate) label: String,
    pub(crate) properties: HashMap<String, Vec<VertexProperty>>,
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
