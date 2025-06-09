use crate::io::graphson::prelude::*;
use crate::io::graphson::tags::{Tag, Type};
use indexmap::IndexSet;

impl V3 {
    fn core_deserializer<'a>(blob: Type<'a>) -> Result<GValue, Error> {
        match blob.tag {
            Tag::Class => blob.value.deserialize::<Self, Class>().map(GValue::from),
            Tag::Date => blob.value.deserialize::<Self, Date>().map(GValue::from),
            Tag::Double => blob.value.deserialize::<Self, Double>().map(GValue::from),
            Tag::Float => blob.value.deserialize::<Self, Float>().map(GValue::from),
            Tag::Integer => blob.value.deserialize::<Self, Integer>().map(GValue::from),
            Tag::List => blob
                .value
                .deserialize::<Self, List<GValue>>()
                .map(GValue::from),
            Tag::Long => blob.value.deserialize::<Self, Long>().map(GValue::from),
            Tag::Map => blob
                .value
                .deserialize::<Self, Map<GValue, GValue>>()
                .map(GValue::from),
            Tag::Set => blob.value.deserialize::<Self, Set>().map(GValue::from),
            Tag::Timestamp => blob
                .value
                .deserialize::<Self, Timestamp>()
                .map(GValue::from),
            Tag::Uuid => blob.value.deserialize::<Self, Uuid>().map(GValue::from),
            Tag::Edge => blob.value.deserialize::<Self, Edge>().map(GValue::from),
            Tag::Path => blob.value.deserialize::<Self, Path>().map(GValue::from),
            Tag::Property => blob.value.deserialize::<Self, Property>().map(GValue::from),
            // Tag::StarGraph => blob.value.deserialize::<Self, StarGraph>().map(GValue::from),
            Tag::TinkerGraph => blob
                .value
                .deserialize::<Self, TinkerGraph>()
                .map(GValue::from),
            // Tag::Tree => blob.value.deserialize::<Self, Tree>().map(GValue::from),
            Tag::Vertex => blob.value.deserialize::<Self, Vertex>().map(GValue::from),
            Tag::VertexProperty => blob
                .value
                .deserialize::<Self, VertexProperty>()
                .map(GValue::from),
            Tag::BulkSet => blob.value.deserialize::<Self, BulkSet>().map(GValue::from),
            // Tag::Bytecode => blob.value.deserialize::<Self, Bytecode>().map(GValue::from),
            // Tag::Cardinality => blob.value.deserialize::<Self, Cardinality>().map(GValue::from),
            // Tag::Column => blob.value.deserialize::<Self, Column>().map(GValue::from),
            // Tag::Direction => blob.value.deserialize::<Self, Direction>().map(GValue::from),
            // Tag::DT => blob.value.deserialize::<Self, DT>().map(GValue::from),
            // Tag::Merge => blob.value.deserialize::<Self, Merge>().map(GValue::from),
            Tag::Metrics => blob.value.deserialize::<Self, Metrics>().map(GValue::from),
            // Tag::Order => blob.value.deserialize::<Self, Order>().map(GValue::from),
            // Tag::P => blob.value.deserialize::<Self, P>().map(GValue::from),
            // Tag::Pop => blob.value.deserialize::<Self, Pop>().map(GValue::from),
            // Tag::Scope => blob.value.deserialize::<Self, Scope>().map(GValue::from),
            // Tag::T => blob.value.deserialize::<Self, T>().map(GValue::from),
            // Tag::TextP => blob.value.deserialize::<Self, TextP>().map(GValue::from),
            // Tag::TraversalExplanation => blob.value.deserialize::<Self, TraversalExplanation>().map(GValue::from),
            Tag::TraversalMetrics => blob
                .value
                .deserialize::<Self, TraversalMetrics>()
                .map(GValue::from),
            // Tag::Traverser => blob.value.deserialize::<Self, Traverser>().map(GValue::from),
            type_tag => Err(Error::Unsupported(type_tag.to_string())),
        }
    }
}

impl Deserializer<GValue> for V3 {
    fn deserialize(val: &Value) -> Result<GValue, Error> {
        match val {
            Value::String(string) => Ok(GValue::from(string)),
            Value::Number(_) => val.deserialize::<Self, Integer>().map(GValue::from),
            Value::Object(_obj) => match val.typed() {
                Ok(blob) => Self::core_deserializer(blob),
                Err(err) => Err(err),
            },
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(Self::deserialize)
                    .collect::<Result<Vec<_>, Error>>()?
                    .into();
                Ok(GValue::List(collection))
            }
            Value::Bool(bool) => Ok(Bool(*bool).into()),
            Value::Null => Ok(GValue::Null),
        }
    }
}

impl Serializer<GValue> for V3 {
    fn serialize(val: &GValue) -> Result<Value, Error> {
        match val {
            GValue::Null => Ok(Value::Null),
            GValue::Bool(v) => v.serialize::<Self>(),
            GValue::Class(v) => v.serialize::<Self>(),
            GValue::Date(v) => v.serialize::<Self>(),
            GValue::Double(v) => v.serialize::<Self>(),
            GValue::Float(v) => v.serialize::<Self>(),
            GValue::Integer(v) => v.serialize::<Self>(),
            GValue::List(v) => v.serialize::<Self>(),
            GValue::Long(v) => v.serialize::<Self>(),
            GValue::Map(v) => v.serialize::<Self>(),
            GValue::Set(v) => v.serialize::<Self>(),
            GValue::String(v) => v.serialize::<Self>(),
            GValue::Timestamp(v) => v.serialize::<Self>(),
            GValue::Uuid(v) => v.serialize::<Self>(),
            GValue::Edge(v) => v.serialize::<Self>(),
            GValue::Path(v) => v.serialize::<Self>(),
            GValue::Property(v) => v.serialize::<Self>(),
            // GValue::StarGraph(v) => v.serialize::<Self>(),
            // GValue::TinkerGraph(v) => v.serialize::<Self>(),
            // GValue::Tree(v) => v.serialize::<Self>(),
            GValue::Vertex(v) => v.serialize::<Self>(),
            GValue::VertexProperty(v) => v.serialize::<Self>(),
            // GValue::Bytecode(v) => v.serialize::<Self>(),
            // GValue::Cardinality(v) => v.serialize::<Self>(),
            // GValue::Column(v) => v.serialize::<Self>(),
            // GValue::Direction(v) => v.serialize::<Self>(),
            // GValue::Order(v) => v.serialize::<Self>(),
            // GValue::Pop(v) => v.serialize::<Self>(),
            // GValue::P(v) => v.serialize::<Self>(),
            // GValue::Scope(v) => v.serialize::<Self>(),
            // GValue::T(v) => v.serialize::<Self>(),
            // GValue::TraversalMetrics(v) => v.serialize::<Self>(),
            // GValue::Traverser(v) => v.serialize::<Self>(),
            // GValue::Int128(v) => v.serialize::<Self>(),
            // GValue::Token(v) => v.serialize::<Self>(),
            // GValue::Metric(v) => v.serialize::<Self>(),
            // GValue::TraversalExplanation(v) => v.serialize::<Self>(),
            // GValue::IntermediateRepr(v) => v.serialize::<Self>(),
            // GValue::TextP(v) => v.serialize::<Self>(),
            // GValue::Geometry(v) => v.serialize::<Self>(),
            // GValue::Merge(v) => v.serialize::<Self>(),
            // GValue::BulkSet(v) => v.serialize::<Self>(),
            gvalue => Err(Error::UnexpectedGValue {
                msg: "We can't serialize this yet".to_string(),
                value: gvalue.clone(),
            }),
        }
    }
}
