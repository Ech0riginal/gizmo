use crate::io::graphson::GraphSONSerializer;
use crate::prelude::{GValue, GraphSON, GremlinResult, ToGValue};
use serde_json::{Value, json};

pub use crate::io::graphson::v2::ser::*;
use crate::io::{V2, V3};

impl GraphSONSerializer for V3 {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        match value {
            GValue::List(_) => list::<Self>(value),
            GValue::Map(_) => map::<Self>(value),
            GValue::List(_) => list::<Self>(value),
            GValue::Set(_) => set::<Self>(value),
            GValue::P(_) => p::<Self>(value),
            GValue::Bytecode(_) => bytecode::<Self>(value),
            GValue::Vertex(_) => vertex::<Self>(value),
            GValue::VertexProperty(_) => vertex_property::<Self>(value),
            GValue::Edge(_) => edge::<Self>(value),
            GValue::Map(_) => map::<Self>(value),
            GValue::TextP(_) => text_p::<Self>(value),
            GValue::Path(_) => path::<Self>(value),
            GValue::Merge(_) => merge(value),
            GValue::T(_) => t(value),
            _ => V2::serialize(value),
        }
    }
}

pub(crate) fn list<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let list = get_value!(value, GValue::List)?;
    let elements: GremlinResult<Vec<Value>> = list.iter().map(|e| S::serialize(e)).collect();
    Ok(json!({
        "@type" : "g:List",
        "@value" : elements?
    }))
}

pub(crate) fn map<S: GraphSON>(value: &GValue) -> GremlinResult<Value> {
    let map = get_value!(value, GValue::Map)?;
    let mut params = vec![];

    for (k, v) in map.iter() {
        if let Ok(key) = S::serialize(&k.clone().into()) {
            if let Ok(value) = S::serialize(&v) {
                params.push(key);
                params.push(value);
            }
        }
    }

    Ok(json!({
        "@type" : "g:Map",
        "@value" : params
    }))
}

pub(crate) fn property<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
    let property = get_value!(value, GValue::Property)?;

    Ok(json!({
        "@type": "g:Property",
        "@value": {
          "key" : S::serialize(&property.label().to_gvalue())?,
          "value" : S::serialize(property.value())?,
        }
    }))
}

// pub(crate) fn options<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
//
// }

// pub(crate) fn vertex_property<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
//     let property = get_value!(value, GValue::VertexProperty)?;
//
//     Ok(json!({
//         "@type": "g:VertexProperty",
//         "@value" : {
//             "id" : S::serialize(&property.id().to_gvalue())?,
//             "value": S::serialize(&property.value())?,
//             "label": S::serialize(&property.label().to_gvalue())?,
//         }
//     }))
// }

// pub(crate) fn edge<S: GraphSONSerializer>(value: &GValue) -> GremlinResult<Value> {
//
// }
