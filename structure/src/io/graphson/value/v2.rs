use crate::io::graphson::prelude::*;
use crate::io::graphson::tags::{Tag, Type};
use indexmap::IndexSet;
use serde_json::Value;

impl V2 {
    fn core_deserializer<'a>(blob: Type<'a>) -> Result<GValue, Error> {
        match blob.tag {
            Tag::Class => blob.value.deserialize::<Self, Class>().map(GValue::from),
            Tag::Date => blob.value.deserialize::<Self, Date>().map(GValue::from),
            Tag::Double => blob.value.deserialize::<Self, Double>().map(GValue::from),
            Tag::Float => blob.value.deserialize::<Self, Float>().map(GValue::from),
            Tag::Integer => blob.value.deserialize::<Self, Integer>().map(GValue::from),
            // Tag::List => blob.value.deserialize::<Self, List>().map(GValue::from),
            Tag::Long => blob.value.deserialize::<Self, Long>().map(GValue::from),
            // Tag::Map => blob.value.deserialize::<Self, Map>().map(GValue::from),
            // Tag::Set => blob.value.deserialize::<Self, Set>().map(GValue::from),
            Tag::Timestamp => blob
                .value
                .deserialize::<Self, Timestamp>()
                .map(GValue::from),
            Tag::Uuid => blob.value.deserialize::<Self, Uuid>().map(GValue::from),
            Tag::Edge => blob.value.deserialize::<Self, Edge>().map(GValue::from),
            Tag::Path => blob.value.deserialize::<Self, Path>().map(GValue::from),
            Tag::Property => blob.value.deserialize::<Self, Property>().map(GValue::from),
            Tag::StarGraph => blob
                .value
                .deserialize::<Self, StarGraph>()
                .map(GValue::from),
            Tag::TinkerGraph => blob
                .value
                .deserialize::<Self, TinkerGraph>()
                .map(GValue::from),
            Tag::Tree => blob.value.deserialize::<Self, Tree>().map(GValue::from),
            Tag::Vertex => blob.value.deserialize::<Self, Vertex>().map(GValue::from),
            Tag::VertexProperty => blob
                .value
                .deserialize::<Self, VertexProperty>()
                .map(GValue::from),
            // Tag::BulkSet => blob.value.deserialize::<Self, BulkSet>().map(GValue::from),
            Tag::Bytecode => blob.value.deserialize::<Self, Bytecode>().map(GValue::from),
            Tag::Cardinality => blob
                .value
                .deserialize::<Self, Cardinality>()
                .map(GValue::from),
            // Tag::Column => blob.value.deserialize::<Self, Column>().map(GValue::from),
            Tag::Direction => blob
                .value
                .deserialize::<Self, Direction>()
                .map(GValue::from),
            // Tag::DT => blob.value.deserialize::<Self, DT>().map(GValue::from),
            // Tag::Merge => blob.value.deserialize::<Self, Merge>().map(GValue::from),
            Tag::Metrics => blob.value.deserialize::<Self, Metrics>().map(GValue::from),
            // Tag::Order => blob.value.deserialize::<Self, Order>().map(GValue::from),
            // Tag::P => blob.value.deserialize::<Self, P>().map(GValue::from),
            // Tag::Pop => blob.value.deserialize::<Self, Pop>().map(GValue::from),
            // Tag::Scope => blob.value.deserialize::<Self, Scope>().map(GValue::from),
            Tag::T => blob.value.deserialize::<Self, T>().map(GValue::from),
            // Tag::TextP => blob.value.deserialize::<Self, TextP>().map(GValue::from),
            // Tag::TraversalExplanation => blob.value.deserialize::<Self, TraversalExplanation>().map(GValue::from),
            Tag::TraversalMetrics => blob
                .value
                .deserialize::<Self, TraversalMetrics>()
                .map(GValue::from),
            Tag::Traverser => blob
                .value
                .deserialize::<Self, Traverser>()
                .map(GValue::from),
            type_tag => Err(Error::Unsupported(type_tag.to_string())),
        }
    }

    fn special_deserializer<'a>(value: &Value) -> Result<GValue, Error> {
        match value {
            val if is_stargraph(val) => value.deserialize::<Self, StarGraph>().map(GValue::from),
            _ => Err(Error::UnexpectedJson {
                msg: "Special case".into(),
                value: value.clone(),
            }),
        }
    }
}

fn is_stargraph(val: &Value) -> bool {
    val.get("starVertex").is_some()
}

impl Deserializer<GValue> for V2 {
    fn deserialize(value: &Value) -> Result<GValue, Error> {
        match value {
            Value::String(string) => Ok(GValue::from(string)),
            Value::Number(_) => value.deserialize::<Self, Integer>().map(GValue::from),
            Value::Object(_obj) => match value.typed() {
                Ok(blob) => Self::core_deserializer(blob),
                Err(err) => match err {
                    Error::Missing(_) => Self::special_deserializer(value),
                    _ => panic!(),
                },
            },
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(Self::deserialize)
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(GValue::List(List(collection)))
            }
            Value::Bool(bool) => Ok(Bool(*bool).into()),
            Value::Null => Ok(GValue::Null),
        }
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
                Err(Error::Unsupported("wat TraversalExplanation".into()))
            }
            GValue::IntermediateRepr(_) => Err(Error::Unsupported("IntermediateRepr".into())),
            GValue::TextP(val) => val.serialize::<Self>(),
            GValue::Geometry(_) => Err(Error::Unsupported("Geometry".into())),
            GValue::Merge(_) => Err(Error::Unsupported("Merge".into())),
            GValue::BulkSet(_) => Err(Error::Unsupported("BulkSet".into())),
        }
    }
}
