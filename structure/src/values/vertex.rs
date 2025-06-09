use crate::VertexProperty;
use crate::{GID, List, Map2};
use indexmap::map::{IntoIter, Iter};
use indexmap::{IndexMap, IndexSet};
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub(crate) id: GID,
    pub(crate) label: String,
    pub(crate) properties: Map2<String, List<VertexProperty>>,
}

impl Vertex {
    pub(crate) fn new<T>(
        id: GID,
        label: T,
        properties: Map2<String, List<VertexProperty>>,
    ) -> Vertex
    where
        T: Into<String>,
    {
        Vertex {
            id,
            label: label.into(),
            properties,
        }
    }

    pub fn id(&self) -> &GID {
        &self.id
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn iter(&self) -> Iter<String, List<VertexProperty>> {
        self.properties.iter()
    }

    pub fn property(&self, key: &str) -> Option<&VertexProperty> {
        self.properties.get(key).and_then(|v| v.get(0))
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
        &self.id == other.id()
    }
}

impl std::hash::Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
