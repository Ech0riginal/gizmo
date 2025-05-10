use crate::structure::*;
use crate::structure::{GID, Vertex};
use std::collections::HashMap;
use std::collections::hash_map::{IntoIter, Iter};
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Edge {
    pub(crate) id: GID,
    pub(crate) label: String,
    pub(crate) in_v: Vertex,
    pub(crate) out_v: Vertex,
    pub(crate) properties: HashMap<String, Box<GValue>>,
}

impl Edge {
    pub(crate) fn new<T>(
        id: GID,
        label: T,
        in_v_id: GID,
        in_v_label: T,
        out_v_id: GID,
        out_v_label: T,
        properties: HashMap<String, Box<GValue>>,
    ) -> Edge
    where
        T: Into<String>,
    {
        Edge {
            id,
            label: label.into(),
            in_v: Vertex::new(in_v_id, in_v_label, HashMap::new()),
            out_v: Vertex::new(out_v_id, out_v_label, HashMap::new()),
            properties,
        }
    }

    pub fn id(&self) -> &GID {
        &self.id
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn in_v(&self) -> &Vertex {
        &self.in_v
    }
    pub fn out_v(&self) -> &Vertex {
        &self.out_v
    }

    pub fn iter(&self) -> Iter<String, Box<GValue>> {
        self.properties.iter()
    }

    pub fn property(&self, key: &str) -> Option<&Box<GValue>> {
        self.properties.get(key)
    }
}

impl IntoIterator for Edge {
    type Item = (String, Box<GValue>);
    type IntoIter = IntoIter<String, Box<GValue>>;
    fn into_iter(self) -> Self::IntoIter {
        self.properties.into_iter()
    }
}

impl std::cmp::Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        &self.id == other.id()
    }
}

impl std::hash::Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
