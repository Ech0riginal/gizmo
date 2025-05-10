use crate::io::{Error, GremlinIO, Serializer};
use crate::io::graphson::types::v2::*;
use crate::io::macros::*;
use crate::structure::*;
use crate::{GValue, GremlinResult};
use serde_json::{Value, json};
use std::collections::HashMap;
use crate::io::serde::Serialize;

pub fn serialize<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    match value {
        // Core
        GValue::Class(_) => class(value),
        GValue::Integer(_) => int32(value),
        GValue::Long(_) => int64(value),
        GValue::Float(_) => float(value),
        GValue::Double(_) => double(value),
        GValue::String(_) => string(value),
        GValue::Date(_) => date(value),
        GValue::Timestamp(_) => timestamp(value),
        GValue::Uuid(_) => uuid(value),
        // Structure
        GValue::Edge(_) => edge::<S>(value),
        GValue::Path(_) => path::<S>(value),
        GValue::Property(_) => property::<S>(value),
        GValue::StarGraph(_) => star_graph::<S>(value),
        GValue::TinkerGraph(_) => tinker_graph::<S>(value),
        GValue::Tree(_) => tree::<S>(value),
        GValue::Vertex(_) => vertex::<S>(value),
        GValue::VertexProperty(_) => vertex_property::<S>(value),
        // Process
        // GValue::Barrier(_) => todo!("v2::barrier"),
        // GValue::Binding(_) => todo!("v2::binding"),
        GValue::Bytecode(_) => bytecode::<S>(value),
        GValue::Cardinality(_) => cardinality(value),
        GValue::Column(_) => column(value),
        GValue::Direction(_) => direction(value),
        // GValue::DT(_) => todo!("v2::dt"),
        // GValue::Lambda(_) => todo!("v2::lambda"),
        GValue::Merge(_) => merge(value),
        // GValue::Metrics(_) => todo!("v2::metrics"),
        // GValue::Operator(_) => todo!("v2::operator"),
        GValue::Order(_) => order(value),
        GValue::P(_) => p::<S>(value),
        // GValue::Pick(_) => todo!("v2::pick"),
        GValue::Pop(_) => pop(value),
        GValue::Scope(_) => scope(value),
        GValue::T(_) => t(value),
        GValue::TextP(_) => text_p::<S>(value),
        GValue::TraversalMetrics(_) => todo!("v2::traversalmetrics"),
        GValue::Traverser(_) => todo!("v2::traverser"),

        GValue::List(_) => list::<S>(value),
        // GValue::Set(_) => set::<Self>(value),
        // GValue::P(_) => p::<Self>(value),

        // GValue::Map(_) => map::<Self>(value),
        // GValue::Bool(_) => bool(value),
        GValue::Null => Ok(serde_json::Value::Null),
        value => panic!("Unsupported type {:?}", value),
    }
}

pub fn double(value: &GValue) -> Result<Value, Error> {
    let double = get_value!(value, GValue::Double)?;
    Ok(json!({
        "@type" : DOUBLE,
        "@value" : **double,
    }))
}

pub fn float(value: &GValue) -> Result<Value, Error> {
    let float = get_value!(value, GValue::Float)?;
    Ok(json!({
        "@type" : FLOAT,
        "@value" : float,
    }))
}

pub fn class(value: &GValue) -> Result<Value, Error> {
    let class = get_value!(value, GValue::Class)?;
    Ok(json!({
        "@type" : CLASS,
        "@value" : **class,
    }))
}

pub fn int32(value: &GValue) -> Result<Value, Error> {
    let int32 = get_value!(value, GValue::Integer)?;
    Ok(json!({
        "@type" : INT,
        "@value" : **val,
    }))
}

pub fn int64(value: &GValue) -> Result<Value, Error> {
    let int64 = get_value!(value, GValue::Long)?;
    Ok(json!({
        "@type" : LONG,
        "@value" : int64,
    }))
}

pub fn string(value: &GValue) -> Result<Value, Error> {
    let string = get_value!(value, GValue::String)?;
    Ok(Value::String(string.clone()))
    // Ok(json!({
    //     "@type" : "g:String",
    //     "@value" : string,
    // }))
}

pub fn uuid(value: &GValue) -> Result<Value, Error> {
    let uuid = get_value!(value, GValue::Uuid)?;
    Ok(json!({
        "@type" : UUID,
        "@value" : uuid.to_string()
    }))
}

pub fn date(value: &GValue) -> Result<Value, Error> {
    let date = get_value!(value, GValue::Date)?;
    let millis = date.timestamp_millis();

    Ok(json!({
        "@type" : DATE,
        "@value" : millis
    }))
}

pub fn timestamp(value: &GValue) -> Result<Value, Error> {
    let ms_since_epoch = get_value!(value, GValue::Timestamp)?.0;
    Ok(json!({
        "@type" : TIMESTAMP,
        "@value" : ms_since_epoch
    }))
}

pub fn list<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let list = get_value!(value, GValue::List)?;
    let elements = list
        .iter()
        .map(S::serialize)
        .collect::<Result<Vec<Value>, Error>>()?;

    Ok(json!(elements))
}

pub fn set<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let list = get_value!(value, GValue::Set)?;
    let elements = list
        .iter()
        .map(S::serialize)
        .collect::<Result<Vec<Value>, Error>>()?;

    Ok(json!({
        "@type" : "g:Set",
        "@value" : elements,
    }))
}

pub fn p<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let p = get_value!(value, GValue::P)?;
    Ok(json!({
        "@type" : P,
        "@value" : {
            "predicate" : p.operator(),
            "value" : S::serialize(p.value())?
        }
    }))
}

pub fn bytecode<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let code = get_value!(value, GValue::Bytecode)?;

    let steps: Result<Vec<Value>, Error> = code
        .steps()
        .iter()
        .map(|m| {
            let mut instruction = vec![];
            instruction.push(Value::String(m.operator.clone()));

            let arguments: Result<Vec<Value>, Error> =
                m.args.iter().map(|a| S::serialize(a)).collect();

            instruction.extend(arguments?);
            Ok(Value::Array(instruction))
        })
        .collect();

    let sources: Result<Vec<Value>, Error> = code
        .sources()
        .iter()
        .map(|m| {
            let mut instruction = vec![];
            instruction.push(Value::String(m.operator.clone()));

            let arguments: Result<Vec<Value>, Error> =
                m.args.iter().map(|a| S::serialize(a)).collect();

            instruction.extend(arguments?);
            Ok(Value::Array(instruction))
        })
        .collect();

    Ok(json!({
        "@type" : BYTECODE,
        "@value" : {
            "step" : steps?,
            "source" : sources?
        }
    }))
}

pub fn tree<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let tree = get_value!(value, GValue::Tree)?;
    let branches = tree
        .branches
        .iter()
        .map(tree_branch::<S>)
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(json!({
        "@type": TREE,
        "@value": branches,
    }))
}

pub fn tree_branch<S: Serializer<GID> + Serializer<GValue>>(value: &Branch) -> Result<Value, Error> {
    Ok(json!({
        "key": S::serialize(&*value.key)?,
        "value": S::serialize(&*value.value)?,
    }))
}

pub fn vertex<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let vertex = get_value!(value, GValue::Vertex)?;
    let mut root = HashMap::<&'static str, Value>::new();
    let mut value = HashMap::<&'static str, Value>::new();

    value.insert("id", S::serialize(vertex.id())?);
    value.insert("label", serde_json::to_value(vertex.label())?);
    if !vertex.properties.is_empty() {
        let properties = vertex
            .iter()
            .map(|(label, properties)| {
                (
                    label.clone(),
                    properties
                        .into_iter()
                        .map(|vp| GValue::VertexProperty(vp.clone()))
                        .flat_map(|v| vertex_property::<S>(&v))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<String, Vec<Value>>>();
        value.insert("properties", serde_json::to_value(&properties)?);
    }
    root.insert("@type", Value::String(VERTEX.into()));
    root.insert("@value", serde_json::to_value(&value)?);

    let json = json!(root);
    let _debug_info = serde_json::to_string_pretty(&json)?;

    Ok(json)
}

pub fn vertex_property<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let property = get_value!(value, GValue::VertexProperty)?;
    let mut root = HashMap::<&'static str, Value>::new();
    let mut value = HashMap::<&'static str, Value>::new();

    value.insert("id", S::serialize(property.id())?);
    value.insert("value", S::serialize(&*property.value)?);
    value.insert("label", serde_json::to_value(&property.label)?);
    if let Some(id) = &property.vertex {
        value.insert("vertex", S::serialize(id)?);
    }
    if let Some(properties) = &property.properties {
        let map = properties
            .iter()
            .map(|(k, v)| (k, S::serialize(v)))
            .filter(|(_, v)| v.is_ok())
            .map(|(k, v)| (k, v.unwrap()))
            .collect::<HashMap<&String, Value>>();
        value.insert("properties", serde_json::to_value(&map)?);
    }

    root.insert("@type", Value::String(VERTEX_PROPERTY.into()));
    root.insert("@value", serde_json::to_value(&value)?);

    let json = json!(root);
    let _debug_info = serde_json::to_string_pretty(&json)?;

    Ok(json)
}
pub fn edge<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    rly_edge::<S>(value, true)
}
pub fn dumbass_edge_in_property<S: GremlinIO>(value: &GValue) -> Result<Value, Error> {
    rly_edge::<S>(value, false)
}
pub fn rly_edge<S: Serializer<GID> + Serializer<GValue>>(
pub fn rly_edge<S: Serializer<GID> + Serializer<GValue>>(
    value: &GValue,
    serialize_labels: bool,
) -> Result<Value, Error> {
    let edge = get_value!(value, GValue::Edge)?;

    let mut value = HashMap::new();
    value.insert("id", S::serialize(edge.id())?);
    value.insert("label", edge.label().serialize::<S>()?);
    if serialize_labels {
        value.insert("inVLabel", S::serialize(&edge.in_v.label().into())?);
        value.insert("outVLabel", S::serialize(&edge.out_v.label().into())?);
    }
    value.insert("inV", S::serialize(&edge.in_v.id().into())?);
    value.insert("outV", S::serialize(&edge.out_v.id().into())?);
    if !edge.properties.is_empty() {
        let properties = edge
            .properties
            .iter()
            .map(|(label, property)| (label, S::serialize(&**property)))
            .filter(|(_, v)| v.is_ok())
            .map(|(k, v)| (k, v.unwrap()))
            .collect::<HashMap<&String, Value>>();
        value.insert("properties", serde_json::to_value(&properties)?);
    }

    Ok(json!({
        "@type": EDGE,
        "@value": value
    }))
}

// deserialize_path
pub fn path<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let path = get_value!(value, GValue::Path)?;

    Ok(json!({
        "@type" : PATH,
        "@value": {
            "labels" : S::serialize(&*path.labels)?,
            "objects" : S::serialize(&*path.objects)?,
        }
    }))
}

pub fn property<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let property = get_value!(value, GValue::Property)?;

    Ok(json!({
        "@type": PROPERTY,
        "@value": {
            "key": property.key,
            "value": S::serialize(&*property.value)?,
            "element": match &*property.element {
                GValue::Edge(_) => dumbass_edge_in_property::<S>(&*property.element)?,
                element => S::serialize(element)?,
            }
        }
    }))
}

pub fn star_graph<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let star = get_value!(value, GValue::StarGraph)?;
    let binding = GValue::Vertex(star.into());
    Ok(json!({
        "starVertex": vertex::<S>(&binding)?
    }))
}

pub fn tinker_graph<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let tinker = get_value!(value, GValue::TinkerGraph)?;
    let vertices = tinker
        .vertices
        .iter()
        .map(Clone::clone)
        .map(GValue::from)
        .map(|gv| vertex::<S>(&gv))
        .collect::<Result<Vec<Value>, Error>>()?;
    let edges = tinker
        .edges
        .iter()
        .map(Clone::clone)
        .map(GValue::from)
        .map(|gv| edge::<S>(&gv))
        .collect::<Result<Vec<Value>, Error>>()?;

    Ok(json!({
        "@type": TINKER_GRAPH,
        "@value": {
            "vertices": vertices,
            "edges": edges,
        }
    }))
}

pub fn t(value: &GValue) -> Result<Value, Error> {
    let t = get_value!(value, GValue::T)?;
    let v = match t {
        T::Id => "id",
        T::Key => "key",
        T::Label => "label",
        T::Value => "value",
    };

    Ok(json!({
        "@type" : T,
        "@value" : v
    }))
}

pub fn scope(value: &GValue) -> Result<Value, Error> {
    let s = get_value!(value, GValue::Scope)?;

    let v = match s {
        Scope::Global => "global",
        Scope::Local => "local",
    };

    Ok(json!({
        "@type" : SCOPE,
        "@value" : v
    }))
}

pub fn order(value: &GValue) -> Result<Value, Error> {
    let order = get_value!(value, GValue::Order)?;

    let v = match order {
        Order::Asc => "asc",
        Order::Desc => "desc",
        Order::Shuffle => "shuffle",
    };

    Ok(json!({
        "@type" : ORDER,
        "@value" : v
    }))
}

pub fn text_p<S: Serializer<GID> + Serializer<GValue>>(value: &GValue) -> Result<Value, Error> {
    let text_p = get_value!(value, GValue::TextP)?;
    Ok(json!({
        "@type" : TEXT_P,
        "@value" : {
            "predicate" : text_p.operator(),
            "value" : S::serialize(text_p.value())?
        }
    }))
}

pub fn pop(value: &GValue) -> Result<Value, Error> {
    let pop = get_value!(value, GValue::Pop)?;
    Ok(json!({
        "@type": POP,
        "@value": *pop.to_string(),
    }))
}

pub fn cardinality(value: &GValue) -> Result<Value, Error> {
    let cardinality = get_value!(value, GValue::Cardinality)?;
    let v = match cardinality {
        Cardinality::List => "list",
        Cardinality::Single => "single",
        Cardinality::Set => "set",
    };
    Ok(json!({
        "@type" : CARDINALITY,
        "@value" : v
    }))
}

pub fn merge(value: &GValue) -> Result<Value, Error> {
    let merge = get_value!(value, GValue::Merge)?;
    let merge_option = match merge {
        Merge::OnCreate => "onCreate",
        Merge::OnMatch => "onMatch",
        Merge::OutV => "outV",
        Merge::InV => "inV",
    };
    Ok(json!({
        "@type" : MERGE,
        "@value" : merge_option
    }))
}

pub fn direction(value: &GValue) -> Result<Value, Error> {
    let direction = get_value!(value, GValue::Direction)?;
    let direction_str = match direction {
        Direction::Out | Direction::From => "OUT",
        Direction::In | Direction::To => "IN",
    };
    Ok(json!({
        "@type" : DIRECTION,
        "@value" : direction_str,
    }))
}

pub fn column(value: &GValue) -> Result<Value, Error> {
    let column = get_value!(value, GValue::Column)?;
    let column = match column {
        crate::structure::Column::Keys => "keys",
        crate::structure::Column::Values => "values",
    };
    Ok(json!({
        "@type" : COLUMN,
        "@value" : column,
    }))
}
