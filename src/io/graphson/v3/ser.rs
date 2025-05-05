pub use crate::io::graphson::v2::ser::*;
use crate::io::{GremlinIO, Serializer, get_value};
use crate::prelude::{GValue, ToGValue};
use crate::{GremlinError, GremlinResult};
use serde_json::{Map, Value, json};

pub fn serialize<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    match value {
        /*
           CLASS => todo!("CLASS serializer"),
           DATE => todo!("DATE serializer"),
           DOUBLE => todo!("DOUBLE serializer"),
           FLOAT => todo!("FLOAT serializer"),
           INT => todo!("INT serializer"),
           LIST => todo!("LIST serializer"),
           LONG => todo!("LONG serializer"),
           MAP => todo!("MAP serializer"),
           SET => todo!("SET serializer"),
           TIMESTAMP => todo!("TIMESTAMP serializer"),
           UUID => todo!("UUID serializer"),

           EDGE => todo!("EDGE serializer"),
           PATH => todo!("PATH serializer"),
           PROPERTY => todo!("PROPERTY serializer"),
           TINKER_GRAPH => todo!("TINKER_GRAPH serializer"),
           VERTEX => todo!("VERTEX serializer"),
           VERTEX_PROPERTY => todo!("VERTEX_PROPERTY serializer"),

           BARRIER => todo!("BARRIER serializer"),
           BINDING => todo!("BINDING serializer"),
           BULK_SET => todo!("BULK_SET serializer"),
           BYTECODE => todo!("BYTECODE serializer"),
           CARDINALITY => todo!("CARDINALITY serializer"),
           COLUMN => todo!("COLUMN serializer"),
           DIRECTION => todo!("DIRECTION serializer"),
           DT => todo!("DT serializer"),
           LAMBDA => todo!("LAMBDA serializer"),
           MERGE => todo!("MERGE serializer"),
           METRICS => todo!("METRICS serializer"),
           OPERATOR => todo!("OPERATOR serializer"),
           ORDER => todo!("ORDER serializer"),
           P => todo!("P serializer"),
           PICK => todo!("PICK serializer"),
           POP => todo!("POP serializer"),
           SCOPE => todo!("SCOPE serializer"),
           T => todo!("T serializer"),
           TEXT_P => todo!("TEXT_P serializer"),
           TRAVERSAL_METRICS => todo!("TRAVERSAL_METRICS serializer"),
           TRAVERSER => todo!("TRAVERSER serializer"),
        */
        GValue::List(_) => list::<S>(value),
        GValue::Map(_) => map::<S>(value),
        GValue::List(_) => list::<S>(value),
        GValue::Set(_) => set::<S>(value),
        GValue::P(_) => p::<S>(value),
        GValue::Bytecode(_) => bytecode::<S>(value),
        GValue::Vertex(_) => vertex::<S>(value),
        GValue::VertexProperty(_) => vertex_property::<S>(value),
        GValue::Edge(_) => edge::<S>(value),
        GValue::Map(_) => map::<S>(value),
        GValue::TextP(_) => text_p::<S>(value),
        GValue::Path(_) => path::<S>(value),
        GValue::Merge(_) => merge(value),
        GValue::T(_) => t(value),
        _ => S::serialize(value),
    }
}

pub(crate) fn list<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let list = get_value!(value, GValue::List)?;
    let elements: GremlinResult<Vec<Value>> = list.iter().map(|e| S::serialize(e)).collect();
    Ok(json!({
        "@type" : "g:List",
        "@value" : elements?
    }))
}

pub fn bool(value: &GValue) -> GremlinResult<Value> {
    let b = get_value!(value, GValue::Bool)?;
    let string = match b {
        true => "true",
        false => "false",
    };
    Ok(serde_json::from_str(string).unwrap())
}

pub fn map<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let map = get_value!(value, GValue::Map)?;
    let mut params = Map::new();

    for (k, v) in map.iter() {
        let key = S::serialize(&k.clone().into())?
            .as_str()
            .ok_or_else(|| GremlinError::Generic(format!("Non-string key value for {:?}", k)))?
            .to_string();
        let value = S::serialize(&v)?;
        params.insert(key, value);
    }

    Ok(json!(params))
}

pub(crate) fn property<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let property = get_value!(value, GValue::Property)?;

    Ok(json!({
        "@type": "g:Property",
        "@value": {
          "key" : S::serialize(&property.label().to_gvalue())?,
          "value" : S::serialize(property.value())?,
        }
    }))
}

// pub(crate) fn options<S: Gremlin>(value: &GValue) -> GremlinResult<Value> {
//
// }

// pub(crate) fn vertex_property<S: Gremlin>(value: &GValue) -> GremlinResult<Value> {
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

// pub(crate) fn edge<S: Gremlin>(value: &GValue) -> GremlinResult<Value> {
//
// }
