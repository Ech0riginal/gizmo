use std::hash::Hasher;

use indexmap::map::IntoIter;

use crate::*;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub(crate) id: GID,
    pub(crate) label: String,
    pub(crate) properties: Map<String, List<VertexProperty>>,
}

crate::obj!(Vertex);
crate::tag!(Vertex);

impl Vertex {
    pub fn id(&self) -> &GID {
        &self.id
    }
}

impl IntoIterator for Vertex {
    type Item = (String, List<VertexProperty>);
    type IntoIter = IntoIter<String, List<VertexProperty>>;
    fn into_iter(self) -> Self::IntoIter {
        self.properties.into_iter()
    }
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
