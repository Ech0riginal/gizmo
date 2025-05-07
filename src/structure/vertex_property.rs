use crate::structure::{GID, GValue, Property};
use std::collections::HashMap;

use crate::GremlinResult;
use crate::conversion::{BorrowFromGValue, FromGValue};

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

    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        match self {
            GProperty::Property(p) => p.take(),
            GProperty::VertexProperty(p) => p.take(),
        }
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFromGValue,
    {
        match self {
            GProperty::Property(p) => p.get(),
            GProperty::VertexProperty(p) => p.get(),
        }
    }

    pub fn label(&self) -> &String {
        match self {
            GProperty::Property(p) => p.label(),
            GProperty::VertexProperty(p) => p.label(),
        }
    }
}
impl FromGValue for GProperty {
    fn from_gvalue(v: GValue) -> GremlinResult<Self> {
        match v {
            GValue::VertexProperty(p) => Ok(GProperty::VertexProperty(p)),
            GValue::Property(p) => Ok(GProperty::Property(p)),
            gvalue => panic!("Value not allowed for a property ({:?})", gvalue),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VertexProperty {
    pub(crate) id: GID,
    pub(crate) value: Box<GValue>,
    pub(crate) vertex: Option<GID>,
    pub(crate) label: String,
    pub(crate) properties: Option<HashMap<String, GValue>>,
}

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

    pub fn take<T>(self) -> GremlinResult<T>
    where
        T: FromGValue,
    {
        T::from_gvalue(*self.value)
    }

    pub fn get<'a, T>(&'a self) -> GremlinResult<&'a T>
    where
        T: BorrowFromGValue,
    {
        T::from_gvalue(&self.value)
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}
