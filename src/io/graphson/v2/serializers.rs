use crate::GValue;
use crate::io::graphson::types::v2::*;
use crate::io::serde::Serialize;
use crate::io::{Args, Error, Request, Serializer, V2};
use crate::structure::*;
use serde_json::{Value, json};
use std::collections::HashMap;
use uuid::Uuid;

impl Serializer<Request> for V2 {
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "request_id": val.id,
            "op": val.op,
            "processor": val.proc,
            "args": val.args.serialize::<Self>()?,
        }))
    }
}
impl Serializer<Args> for V2 {
    fn serialize(value: &Args) -> Result<Value, Error> {
        todo!()
    }
}
impl Serializer<GID> for V2 {
    fn serialize(val: &GID) -> Result<Value, Error> {
        let val: GValue = val.into();
        val.serialize::<Self>()
    }
}
impl Serializer<GValue> for V2 {
    fn serialize(val: &GValue) -> Result<Value, Error> {
        match val {
            GValue::Null => Ok(Value::Null),
            GValue::Bool(val) => val.serialize::<Self>(),
            GValue::Class(val) => val.serialize::<Self>(),
            GValue::Date(val) => val.serialize::<Self>(),
            GValue::Double(val) => val.serialize::<Self>(),
            GValue::Float(val) => val.serialize::<Self>(),
            GValue::Integer(val) => val.serialize::<Self>(),
            GValue::List(val) => val.serialize::<Self>(),
            GValue::Long(val) => val.serialize::<Self>(),
            GValue::Map(val) => val.serialize::<Self>(),
            GValue::Set(val) => val.serialize::<Self>(),
            GValue::String(val) => val.serialize::<Self>(),
            GValue::Timestamp(val) => val.serialize::<Self>(),
            GValue::Uuid(val) => val.serialize::<Self>(),
            GValue::Edge(val) => val.serialize::<Self>(),
            GValue::Path(val) => val.serialize::<Self>(),
            GValue::Property(val) => val.serialize::<Self>(),
            GValue::StarGraph(val) => val.serialize::<Self>(),
            GValue::TinkerGraph(val) => val.serialize::<Self>(),
            GValue::Tree(val) => val.serialize::<Self>(),
            GValue::Vertex(val) => val.serialize::<Self>(),
            GValue::VertexProperty(val) => val.serialize::<Self>(),
            GValue::Bytecode(val) => val.serialize::<Self>(),
            GValue::Cardinality(val) => val.serialize::<Self>(),
            GValue::Column(val) => val.serialize::<Self>(),
            GValue::Direction(val) => val.serialize::<Self>(),
            GValue::Order(val) => val.serialize::<Self>(),
            GValue::Pop(val) => val.serialize::<Self>(),
            GValue::P(val) => val.serialize::<Self>(),
            GValue::Scope(val) => val.serialize::<Self>(),
            GValue::T(val) => val.serialize::<Self>(),
            GValue::TraversalMetrics(val) => val.serialize::<Self>(),
            GValue::Traverser(val) => val.serialize::<Self>(),
            GValue::Int128(_) => Err(Error::Unsupported("Int128".into())),
            GValue::Token(_) => Err(Error::Unsupported("Token".into())),
            GValue::Metric(val) => val.serialize::<Self>(),
            GValue::TraversalExplanation(_) => {
                Err(Error::Unsupported("TraversalExplanation".into()))
            }
            GValue::IntermediateRepr(_) => Err(Error::Unsupported("IntermediateRepr".into())),
            GValue::TextP(val) => val.serialize::<Self>(),
            GValue::Geometry(_) => Err(Error::Unsupported("Geometry".into())),
            GValue::Merge(_) => Err(Error::Unsupported("Merge".into())),
            GValue::BulkSet(_) => Err(Error::Unsupported("BulkSet".into())),
        }
    }
}
impl Serializer<Bool> for V2 {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        Ok(Value::Bool(**val))
    }
}
impl Serializer<Class> for V2 {
    fn serialize(val: &Class) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : CLASS,
            "@value" : **val,
        }))
    }
}
impl Serializer<Date> for V2 {
    fn serialize(val: &Date) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : DATE,
            "@value" : val.timestamp_millis()
        }))
    }
}
impl Serializer<Double> for V2 {
    fn serialize(val: &Double) -> Result<Value, Error> {
        Ok(json!({
            "@type" : DOUBLE,
            "@value" : **val,
        }))
    }
}
impl Serializer<Float> for V2 {
    fn serialize(val: &Float) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : FLOAT,
            "@value" : **val,
        }))
    }
}
impl Serializer<Integer> for V2 {
    fn serialize(val: &Integer) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : INT,
            "@value" : **val,
        }))
    }
}
impl Serializer<List> for V2 {
    fn serialize(val: &List) -> Result<Value, Error> {
        let elements = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, Error>>()?;
        Ok(json!(elements))
    }
}
impl Serializer<Long> for V2 {
    fn serialize(val: &Long) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : LONG,
            "@value" : **val,
        }))
    }
}
impl Serializer<Map> for V2 {
    fn serialize(val: &Map) -> Result<Value, Error> {
        todo!()
    }
}
impl Serializer<Timestamp> for V2 {
    fn serialize(val: &Timestamp) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type": TIMESTAMP,
            "@value": val.0,
        }))
    }
}
impl Serializer<Uuid> for V2 {
    fn serialize(val: &Uuid) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : UUID,
            "@value" : val.to_string()
        }))
    }
}
impl Serializer<Edge> for V2 {
    fn serialize(val: &Edge) -> Result<serde_json::Value, Error> {
        serialize_edge::<Self>(val, true)
    }
}
impl Serializer<Path> for V2 {
    fn serialize(val: &Path) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : PATH,
            "@value": {
                "labels" : (&*val.labels).serialize::<Self>()?,
                "objects" : (&*val.objects).serialize::<Self>()?,
            }
        }))
    }
}
impl Serializer<Property> for V2 {
    fn serialize(val: &Property) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type": PROPERTY,
            "@value": {
                "key": val.key,
                "value": (&*val.value).serialize::<Self>()?,
                "element": match &*val.element {
                    GValue::Edge(edge) => serialize_edge::<Self>(edge, false)?,
                    element => element.serialize::<Self>()?,
                }
            }
        }))
    }
}
fn serialize_edge<S>(edge: &Edge, serialize_labels: bool) -> Result<Value, Error>
where
    S: Serializer<GID>,
    S: Serializer<GValue>,
    S: Serializer<String>,
{
    let mut value = HashMap::new();
    value.insert("id", edge.id().serialize::<S>()?);
    value.insert("label", edge.label().serialize::<S>()?);
    if serialize_labels {
        value.insert("inVLabel", edge.in_v.label().serialize::<S>()?);
        value.insert("outVLabel", edge.out_v.label().serialize::<S>()?);
    }
    value.insert("inV", edge.in_v.id().serialize::<S>()?);
    value.insert("outV", edge.out_v.id().serialize::<S>()?);
    if !edge.properties.is_empty() {
        let properties = edge
            .properties
            .iter()
            .map(|(label, property)| (label, (&**property).serialize::<S>()))
            .map(|(label, result)| match result {
                Ok(value) => Ok((label, value)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter()
            .collect::<HashMap<&String, Value>>();
        value.insert("properties", serde_json::to_value(&properties)?);
    }

    Ok(json!({
        "@type": EDGE,
        "@value": value
    }))
}
impl Serializer<StarGraph> for V2 {
    fn serialize(val: &StarGraph) -> Result<serde_json::Value, Error> {
        let binding = GValue::Vertex(val.into());
        Ok(json!({"starVertex": binding.serialize::<Self>()?,}))
    }
}
impl Serializer<TinkerGraph> for V2 {
    fn serialize(val: &TinkerGraph) -> Result<serde_json::Value, Error> {
        let vertices = val
            .vertices
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?;
        let edges = val
            .edges
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(json!({
            "@type": TINKER_GRAPH,
            "@value": {
                "vertices": vertices,
                "edges": edges,
            }
        }))
    }
}
impl Serializer<Tree> for V2 {
    fn serialize(val: &Tree) -> Result<serde_json::Value, Error> {
        let branches = val
            .branches
            .iter()
            .map(|b| b.serialize::<Self>())
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(json!({
            "@type": TREE,
            "@value": branches,
        }))
    }
}
impl Serializer<Branch> for V2 {
    fn serialize(val: &Branch) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "key": (&*val.key).serialize::<Self>()?,
            "value": (&*val.value).serialize::<Self>()?,
        }))
    }
}
impl Serializer<Vertex> for V2 {
    fn serialize(val: &Vertex) -> Result<serde_json::Value, Error> {
        let mut root = HashMap::<&'static str, Value>::new();
        let mut value = HashMap::<&'static str, Value>::new();

        value.insert("id", val.id().serialize::<Self>()?);
        value.insert("label", serde_json::to_value(val.label())?);
        if !val.properties.is_empty() {
            let properties = val
                .iter()
                .map(|(label, properties)| {
                    (
                        label.clone(),
                        properties
                            .into_iter()
                            .flat_map(|vp| vp.serialize::<Self>())
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
}
// type VertexProperties = HashMap<String, Vec<VertexProperty>>;
// impl Serializer<VertexProperties> for V2 {
//     fn serialize(val: &VertexProperties) -> Result<serde_json::Value, Error> { todo!() }
// }
impl Serializer<VertexProperty> for V2 {
    fn serialize(val: &VertexProperty) -> Result<serde_json::Value, Error> {
        let mut root = HashMap::<&'static str, Value>::new();
        let mut value = HashMap::<&'static str, Value>::new();

        value.insert("id", val.id().serialize::<Self>()?);
        value.insert("value", (&*val.value).serialize::<Self>()?);
        value.insert("label", serde_json::to_value(&val.label)?);
        if let Some(id) = &val.vertex {
            value.insert("vertex", id.serialize::<Self>()?);
        }
        if let Some(properties) = &val.properties {
            let map = properties
                .iter()
                .map(|(k, v)| (k, v.serialize::<Self>()))
                .map(|(k, result)| match result {
                    Ok(v) => Ok((k, v)),
                    Err(e) => Err(e),
                })
                .collect::<Result<HashMap<&String, Value>, Error>>()?;
            value.insert("properties", serde_json::to_value(&map)?);
        }

        root.insert("@type", Value::String(VERTEX_PROPERTY.into()));
        root.insert("@value", serde_json::to_value(&value)?);

        let json = json!(root);

        Ok(json)
    }
}
impl Serializer<Bytecode> for V2 {
    fn serialize(val: &Bytecode) -> Result<Value, Error> {
        let steps: Result<Vec<Value>, Error> = val
            .steps()
            .iter()
            .map(|m| {
                let mut instruction = vec![];
                instruction.push(Value::String(m.operator.clone()));

                let arguments: Result<Vec<Value>, Error> =
                    m.args.iter().map(|a| a.serialize::<Self>()).collect();

                instruction.extend(arguments?);
                Ok(Value::Array(instruction))
            })
            .collect();

        let sources: Result<Vec<Value>, Error> = val
            .sources()
            .iter()
            .map(|m| {
                let mut instruction = vec![];
                instruction.push(Value::String(m.operator.clone()));

                let arguments: Result<Vec<Value>, Error> =
                    m.args.iter().map(|a| a.serialize::<Self>()).collect();

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
}
impl Serializer<Cardinality> for V2 {
    fn serialize(val: &Cardinality) -> Result<Value, Error> {
        let str = match val {
            Cardinality::List => "list",
            Cardinality::Set => "set",
            Cardinality::Single => "single",
        };
        Ok(json!({
            "@type": CARDINALITY,
            "@value": str,
        }))
    }
}
impl Serializer<Column> for V2 {
    fn serialize(val: &Column) -> Result<Value, Error> {
        Ok(json!({
            "@type" : COLUMN,
            "@value" : match val {
                Column::Keys => "keys",
                Column::Values => "values",
            },
        }))
    }
}
impl Serializer<Direction> for V2 {
    fn serialize(val: &Direction) -> Result<serde_json::Value, Error> {
        let direction_str = match val {
            Direction::Out | Direction::From => "OUT",
            Direction::In | Direction::To => "IN",
        };
        Ok(json!({
            "@type" : DIRECTION,
            "@value" : direction_str,
        }))
    }
}
impl Serializer<Order> for V2 {
    fn serialize(val: &Order) -> Result<Value, Error> {
        let str = match val {
            Order::Asc => "asc",
            Order::Desc => "desc",
            Order::Shuffle => "shuffle",
        };
        Ok(json!({
            "@type": ORDER,
            "@value": str,
        }))
    }
}
impl Serializer<Pop> for V2 {
    fn serialize(val: &Pop) -> Result<Value, Error> {
        let str = match val {
            Pop::All => "all",
            Pop::First => "first",
            Pop::Last => "last",
            Pop::Mixed => "mixed",
        };
        Ok(json!({
            "@type": POP,
            "@value": str,
        }))
    }
}
impl Serializer<P> for V2 {
    fn serialize(val: &P) -> Result<Value, Error> {
        Ok(json!({
            "@type": P,
            "@value": {
                "predicate": val.operator,
                "value": (&*val.value).serialize::<Self>()?
            }
        }))
    }
}
impl Serializer<Scope> for V2 {
    fn serialize(val: &Scope) -> Result<Value, Error> {
        let v = match val {
            Scope::Global => "global",
            Scope::Local => "local",
        };

        Ok(json!({
            "@type" : SCOPE,
            "@value" : v
        }))
    }
}
impl Serializer<T> for V2 {
    fn serialize(val: &T) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type": T,
            "@value": match val {
                T::Id => "id",
                T::Key => "key",
                T::Label => "label",
                T::Value => "value",
            }
        }))
    }
}
impl Serializer<TextP> for V2 {
    fn serialize(val: &TextP) -> Result<Value, Error> {
        Ok(json!({
            "@type" : TEXT_P,
            "@value" : {
                "predicate" : val.operator(),
                "value" : val.value().serialize::<Self>()?
            }
        }))
    }
}
impl Serializer<Metrics> for V2 {
    fn serialize(val: &Metrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
impl Serializer<HashMap<String, GValue>> for V2 {
    fn serialize(val: &HashMap<String, GValue>) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
impl Serializer<Set> for V2 {
    fn serialize(val: &Set) -> Result<Value, Error> {
        let elements = val
            .iter()
            .map(|v| v.serialize::<Self>())
            .collect::<Result<Vec<Value>, Error>>()?;

        Ok(json!({
            "@type" : "g:Set",
            "@value" : elements,
        }))
    }
}
impl Serializer<String> for V2 {
    fn serialize(val: &String) -> Result<serde_json::Value, Error> {
        Ok(json!(val))
    }
}
impl Serializer<TraversalMetrics> for V2 {
    fn serialize(val: &TraversalMetrics) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
impl Serializer<Traverser> for V2 {
    fn serialize(val: &Traverser) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
