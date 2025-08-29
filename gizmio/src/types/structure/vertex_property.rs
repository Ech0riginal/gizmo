use std::hash::{Hash, Hasher};

use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GProperty {
    VertexProperty(VertexProperty),
    Property(Property),
}

impl GProperty {
    pub fn value(&self) -> &GValue {
        match self {
            GProperty::Property(p) => p.value(),
            GProperty::VertexProperty(p) => p.value(),
        }
    }

    pub fn label(&self) -> &String {
        match self {
            GProperty::Property(p) => p.label(),
            GProperty::VertexProperty(p) => p.label(),
        }
    }
}
// impl TryFrom<GValue> for GProperty {
//     type Error = Error;
//
//     fn try_from(v: GValue) -> GremlinResult<Self> {
//         match v {
//             GValue::VertexProperty(p) => Ok(GProperty::VertexProperty(p)),
//             GValue::Property(p) => Ok(GProperty::Property(p)),
//             gvalue => Err(Error::Cast(
//                 gvalue.to_string(),
//                 "GProperty".to_string()
//             )),
//         }
//     }
// }

impl From<GValue> for GProperty {
    fn from(value: GValue) -> Self {
        match value {
            GValue::VertexProperty(p) => GProperty::VertexProperty(p),
            GValue::Property(p) => GProperty::Property(p),
            _ => panic!("Unexpected value casting to GProperty!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VertexProperty {
    pub(crate) id: GID,
    pub(crate) value: Box<GValue>,
    pub(crate) vertex: Option<GID>,
    pub(crate) label: String,
    pub(crate) properties: Option<Map<String, GValue>>,
}

crate::obj!(VertexProperty);
crate::tag!(VertexProperty);

impl VertexProperty {
    pub fn new<G, T, GT>(id: G, label: T, value: GT) -> VertexProperty
    where
        G: Into<GID>,
        T: Into<String>,
        GT: Into<GValue>,
    {
        VertexProperty {
            id: id.into(),
            value: Box::new(value.into()),
            vertex: None,
            label: label.into(),
            properties: Default::default(),
        }
    }

    pub fn id(&self) -> &GID {
        &self.id
    }

    pub fn value(&self) -> &GValue {
        &self.value
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}
impl Eq for VertexProperty {}
impl Hash for VertexProperty {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.label.hash(state);
        self.value.hash(state);

        if let Some(id) = &self.vertex {
            id.hash(state);
        }
    }
}
