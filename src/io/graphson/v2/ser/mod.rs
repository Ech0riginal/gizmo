use crate::GremlinResult;
use crate::io::graphson::types::v2::*;
use crate::io::macros::*;
use crate::io::Serializer;
use crate::prelude::{
    Cardinality, Direction, GValue, Merge, T,
    traversal::{Order, Scope},
};
use crate::structure::Branch;
use serde_json::{Value, json};
use std::collections::HashMap;

pub fn serialize<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    match value {
        // Core
        GValue::Class(_) => class(value),
        GValue::Int32(_) => int32(value),
        GValue::Int64(_) => int64(value),
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

pub fn double(value: &GValue) -> GremlinResult<Value> {
    let double = get_value!(value, GValue::Double)?;
    Ok(json!({
        "@type" : DOUBLE,
        "@value" : double,
    }))
}

pub fn float(value: &GValue) -> GremlinResult<Value> {
    let float = get_value!(value, GValue::Float)?;
    Ok(json!({
        "@type" : FLOAT,
        "@value" : float,
    }))
}

pub fn class(value: &GValue) -> GremlinResult<Value> {
    let class = get_value!(value, GValue::Class)?;
    Ok(json!({
        "@type" : CLASS,
        "@value" : class,
    }))
}

pub fn int32(value: &GValue) -> GremlinResult<Value> {
    let int32 = get_value!(value, GValue::Int32)?;
    Ok(json!({
        "@type" : INT,
        "@value" : int32,
    }))
}

pub fn int64(value: &GValue) -> GremlinResult<Value> {
    let int64 = get_value!(value, GValue::Int64)?;
    Ok(json!({
        "@type" : LONG,
        "@value" : int64,
    }))
}

pub fn string(value: &GValue) -> GremlinResult<Value> {
    let string = get_value!(value, GValue::String)?;
    Ok(Value::String(string.clone()))
    // Ok(json!({
    //     "@type" : "g:String",
    //     "@value" : string,
    // }))
}

pub fn uuid(value: &GValue) -> GremlinResult<Value> {
    let uuid = get_value!(value, GValue::Uuid)?;
    Ok(json!({
        "@type" : UUID,
        "@value" : uuid.to_string()
    }))
}

pub fn date(value: &GValue) -> GremlinResult<Value> {
    let date = get_value!(value, GValue::Date)?;
    let millis = date.timestamp_millis();

    Ok(json!({
        "@type" : DATE,
        "@value" : millis
    }))
}

pub fn timestamp(value: &GValue) -> GremlinResult<Value> {
    let date = get_value!(value, GValue::Timestamp)?;
    let millis = date.timestamp_millis();
    Ok(json!({
        "@type" : TIMESTAMP,
        "@value" : millis
    }))
}

pub fn list<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let list = get_value!(value, GValue::List)?;
    let elements = list
        .iter()
        .map(S::serialize)
        .collect::<GremlinResult<Vec<Value>>>()?;

    Ok(json!(elements))
}

pub fn set<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let list = get_value!(value, GValue::Set)?;
    let elements = list
        .iter()
        .map(S::serialize)
        .collect::<GremlinResult<Vec<Value>>>()?;

    Ok(json!({
        "@type" : "g:Set",
        "@value" : elements,
    }))
}

pub fn p<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let p = get_value!(value, GValue::P)?;
    Ok(json!({
        "@type" : P,
        "@value" : {
            "predicate" : p.operator(),
            "value" : S::serialize(p.value())?
        }
    }))
}

pub fn bytecode<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let code = get_value!(value, GValue::Bytecode)?;

    let steps: GremlinResult<Vec<Value>> = code
        .steps()
        .iter()
        .map(|m| {
            let mut instruction = vec![];
            instruction.push(Value::String(m.operator().clone()));

            let arguments: GremlinResult<Vec<Value>> =
                m.args().iter().map(|a| S::serialize(a)).collect();

            instruction.extend(arguments?);
            Ok(Value::Array(instruction))
        })
        .collect();

    let sources: GremlinResult<Vec<Value>> = code
        .sources()
        .iter()
        .map(|m| {
            let mut instruction = vec![];
            instruction.push(Value::String(m.operator().clone()));

            let arguments: GremlinResult<Vec<Value>> =
                m.args().iter().map(|a| S::serialize(a)).collect();

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

pub fn tree<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let tree = get_value!(value, GValue::Tree)?;
    let branches = tree
        .branches
        .iter()
        .map(tree_branch::<S>)
        .collect::<GremlinResult<Vec<_>>>()?;
    Ok(json!({
        "@type": TREE,
        "@value": branches,
    }))
}

pub fn tree_branch<S: Serializer<GValue>>(value: &Branch) -> GremlinResult<Value> {
    Ok(json!({
        "key": S::serialize(&value.key)?,
        "value": S::serialize(&value.value)?,
    }))
}

pub fn vertex<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let vertex = get_value!(value, GValue::Vertex)?;
    let mut root = HashMap::<&'static str, Value>::new();
    let mut value = HashMap::<&'static str, Value>::new();

    value.insert("id", S::serialize(&vertex.id().to_gvalue())?);
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

pub fn vertex_property<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let property = get_value!(value, GValue::VertexProperty)?;
    let mut root = HashMap::<&'static str, Value>::new();
    let mut value = HashMap::<&'static str, Value>::new();

    value.insert("id", S::serialize(&property.id().to_gvalue())?);
    value.insert("value", S::serialize(&*property.value)?);
    value.insert("label", serde_json::to_value(&property.label)?);
    if let Some(id) = &property.vertex {
        value.insert("vertex", S::serialize(&id.to_gvalue())?);
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
pub fn edge<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    rly_edge::<S>(value, true)
}
pub fn dumbass_edge_in_property<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    rly_edge::<S>(value, false)
}
pub fn rly_edge<S: Serializer<GValue>>(
    value: &GValue,
    serialize_labels: bool,
) -> GremlinResult<Value> {
    let edge = get_value!(value, GValue::Edge)?;

    let mut value = HashMap::new();
    value.insert("id", S::serialize(&edge.id().to_gvalue())?);
    value.insert("label", S::serialize(&edge.label().to_gvalue())?);
    if serialize_labels {
        value.insert("inVLabel", S::serialize(&edge.in_v.label().to_gvalue())?);
        value.insert("outVLabel", S::serialize(&edge.out_v.label().to_gvalue())?);
    }
    value.insert("inV", S::serialize(&edge.in_v.id().to_gvalue())?);
    value.insert("outV", S::serialize(&edge.out_v.id().to_gvalue())?);
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
pub fn path<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let path = get_value!(value, GValue::Path)?;

    Ok(json!({
        "@type" : PATH,
        "@value": {
            "labels" : S::serialize(&path.labels)?,
            "objects" : S::serialize(&path.objects)?,
        }
    }))
}

pub fn property<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
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

pub fn star_graph<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let star = get_value!(value, GValue::StarGraph)?;
    let binding = GValue::Vertex(star.into());
    Ok(json!({
        "starVertex": vertex::<S>(&binding)?
    }))
}

pub fn tinker_graph<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let tinker = get_value!(value, GValue::TinkerGraph)?;
    let vertices = tinker
        .vertices
        .iter()
        .map(Clone::clone)
        .map(GValue::from)
        .map(|gv| vertex::<S>(&gv))
        .collect::<GremlinResult<Vec<Value>>>()?;
    let edges = tinker
        .edges
        .iter()
        .map(Clone::clone)
        .map(GValue::from)
        .map(|gv| edge::<S>(&gv))
        .collect::<GremlinResult<Vec<Value>>>()?;

    Ok(json!({
        "@type": TINKER_GRAPH,
        "@value": {
            "vertices": vertices,
            "edges": edges,
        }
    }))
}

pub fn t(value: &GValue) -> GremlinResult<Value> {
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

pub fn scope(value: &GValue) -> GremlinResult<Value> {
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

pub fn order(value: &GValue) -> GremlinResult<Value> {
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

pub fn text_p<S: Serializer<GValue>>(value: &GValue) -> GremlinResult<Value> {
    let text_p = get_value!(value, GValue::TextP)?;
    Ok(json!({
        "@type" : TEXT_P,
        "@value" : {
            "predicate" : text_p.operator(),
            "value" : S::serialize(text_p.value())?
        }
    }))
}

pub fn pop(value: &GValue) -> GremlinResult<Value> {
    let pop = get_value!(value, GValue::Pop)?;
    Ok(json!({
        "@type": POP,
        "@value": *pop.to_string(),
    }))
}

pub fn cardinality(value: &GValue) -> GremlinResult<Value> {
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

pub fn merge(value: &GValue) -> GremlinResult<Value> {
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

pub fn direction(value: &GValue) -> GremlinResult<Value> {
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

pub fn column(value: &GValue) -> GremlinResult<Value> {
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
