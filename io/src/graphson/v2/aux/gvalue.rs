use crate::graphson::prelude::*;
use crate::graphson::tags::{Tag, Type, Typed};
use serde_json::Value;



impl<D: Dialect> GraphsonSerializer<GValue, D> for GraphSON<V2> {
    fn serialize(val: &GValue) -> Result<Value, Error> {
        match val {
            GValue::Null => Ok(Value::Null),
            GValue::Bool(val) => val.serialize::<Self, D>(),
            GValue::Class(val) => val.serialize::<Self, D>(),
            GValue::Date(val) => val.serialize::<Self, D>(),
            GValue::Double(val) => val.serialize::<Self, D>(),
            GValue::Float(val) => val.serialize::<Self, D>(),
            GValue::Integer(val) => val.serialize::<Self, D>(),
            GValue::List(val) => val.serialize::<Self, D>(),
            GValue::Long(val) => val.serialize::<Self, D>(),
            GValue::Map(val) => val.serialize::<Self, D>(),
            GValue::Set(val) => val.serialize::<Self, D>(),
            GValue::String(val) => val.serialize::<Self, D>(),
            GValue::Timestamp(val) => val.serialize::<Self, D>(),
            GValue::Uuid(val) => val.serialize::<Self, D>(),
            GValue::Edge(val) => val.serialize::<Self, D>(),
            GValue::Path(val) => val.serialize::<Self, D>(),
            GValue::Property(val) => val.serialize::<Self, D>(),
            GValue::StarGraph(val) => val.serialize::<Self, D>(),
            GValue::TinkerGraph(val) => val.serialize::<Self, D>(),
            GValue::Tree(val) => val.serialize::<Self, D>(),
            GValue::Vertex(val) => val.serialize::<Self, D>(),
            GValue::VertexProperty(val) => val.serialize::<Self, D>(),
            // GValue::Bytecode(val) => val.serialize::<Self, D>(),
            // GValue::Cardinality(val) => val.serialize::<Self, D>(),
            // GValue::Column(val) => val.serialize::<Self, D>(),
            // GValue::Direction(val) => val.serialize::<Self, D>(),
            // GValue::Order(val) => val.serialize::<Self, D>(),
            // GValue::Pop(val) => val.serialize::<Self, D>(),
            // GValue::P(val) => val.serialize::<Self, D>(),
            // GValue::Scope(val) => val.serialize::<Self, D>(),
            // GValue::T(val) => val.serialize::<Self, D>(),
            // GValue::TraversalMetrics(val) => val.serialize::<Self, D>(),
            // GValue::Traverser(val) => val.serialize::<Self, D>(),
            // GValue::Int128(_) => Err(Error::Unsupported {
            //     tag: Tag::Int128.to_string(),
            //     location: location!(),
            // }),
            // GValue::Metrics(val) => val.serialize::<Self, D>(),
            // GValue::TextP(val) => val.serialize::<Self, D>(),
            value => Err(Error::Unsupported {
                tag: format!("{}", value),
                location: location!(),
            }),
        }
    }
}

impl<D: Dialect> GraphsonDeserializer<GValue, D> for GraphSON<V2> {
    fn deserialize(value: &Value) -> Result<GValue, Error> {
        match value {
            Value::String(string) => Ok(GValue::from(string)),
            Value::Number(_) => value.deserialize::<Self, D, Integer>().map(GValue::from),
            Value::Object(_obj) => match value.typed() {
                Ok(blob) => deserialize::<D>(blob),
                Err(err) => match err {
                    Error::Missing { .. } => deserialize_variant::<D>(value),
                    _ => panic!(),
                },
            },
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(|v| v.deserialize::<Self, D, GValue>())
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(GValue::List(List(collection)))
            }
            Value::Bool(bool) => Ok(Bool(*bool).into()),
            Value::Null => Ok(GValue::Null),
        }
    }
}

fn is_stargraph(val: &Value) -> bool {
    val.get("starVertex").is_some()
}

fn deserialize<'a, D: Dialect>(blob: Type<'a>) -> Result<GValue, Error> {
    match blob.tag {
        Tag::Class => blob.value.deserialize::<GraphSON<V2>, D, Class>().map(GValue::from),
        Tag::Date => blob.value.deserialize::<GraphSON<V2>, D, Date>().map(GValue::from),
        Tag::Double => blob.value.deserialize::<GraphSON<V2>, D, Double>().map(GValue::from),
        Tag::Float => blob.value.deserialize::<GraphSON<V2>, D, Float>().map(GValue::from),
        Tag::Integer => blob.value.deserialize::<GraphSON<V2>, D, Integer>().map(GValue::from),
        // Tag::List => blob.value.deserialize::<GraphSON<V2>, D, List<GValue>>().map(GValue::from),
        Tag::Long => blob.value.deserialize::<GraphSON<V2>, D, Long>().map(GValue::from),
        // Tag::Map => blob.value.deserialize::<Self, Map>().map(GValue::from),
        // Tag::Set => blob.value.deserialize::<Self, Set>().map(GValue::from),
        Tag::Timestamp => blob.value.deserialize::<GraphSON<V2>, D, Timestamp>().map(GValue::from),
        Tag::Uuid => blob.value.deserialize::<GraphSON<V2>, D, Uuid>().map(GValue::from),
        Tag::Edge => blob.value.deserialize::<GraphSON<V2>, D, Edge>().map(GValue::from),
        Tag::Path => blob.value.deserialize::<GraphSON<V2>, D, Path>().map(GValue::from),
        Tag::Property => blob.value.deserialize::<GraphSON<V2>, D, Property>().map(GValue::from),
        Tag::StarGraph => blob.value.deserialize::<GraphSON<V2>, D, StarGraph>().map(GValue::from),
        Tag::TinkerGraph => blob
            .value
            .deserialize::<GraphSON<V2>, D, TinkerGraph>()
            .map(GValue::from),
        Tag::Tree => blob.value.deserialize::<GraphSON<V2>, D, Tree>().map(GValue::from),
        Tag::Vertex => blob.value.deserialize::<GraphSON<V2>, D, Vertex>().map(GValue::from),
        Tag::VertexProperty => blob
            .value
            .deserialize::<GraphSON<V2>, D, VertexProperty>()
            .map(GValue::from),
        // Tag::BulkSet => blob.value.deserialize::<Self, BulkSet>().map(GValue::from),
        // Tag::Bytecode => blob.value.deserialize::<GraphSON<V2>, D, Bytecode>().map(GValue::from),
        // Tag::Cardinality => blob
        //     .value
        //     .deserialize::<GraphSON<V2>, D, Cardinality>()
        //     .map(GValue::from),
        // Tag::Column => blob.value.deserialize::<Self, Column>().map(GValue::from),
        // Tag::Direction => blob.value.deserialize::<GraphSON<V2>, D, Direction>().map(GValue::from),
        // Tag::DT => blob.value.deserialize::<Self, DT>().map(GValue::from),
        // Tag::Merge => blob.value.deserialize::<Self, Merge>().map(GValue::from),
        // Tag::Metrics => blob.value.deserialize::<GraphSON<V2>, D, Metrics>().map(GValue::from),
        // Tag::Order => blob.value.deserialize::<Self, Order>().map(GValue::from),
        // Tag::P => blob.value.deserialize::<Self, P>().map(GValue::from),
        // Tag::Pop => blob.value.deserialize::<Self, Pop>().map(GValue::from),
        // Tag::Scope => blob.value.deserialize::<Self, Scope>().map(GValue::from),
        // Tag::T => blob.value.deserialize::<GraphSON<V2>, D, T>().map(GValue::from),
        // Tag::TextP => blob.value.deserialize::<Self, TextP>().map(GValue::from),
        // Tag::TraversalExplanation => blob.value.deserialize::<Self, TraversalExplanation>().map(GValue::from),
        // Tag::TraversalMetrics => blob
        //     .value
        //     .deserialize::<GraphSON<V2>, D, TraversalMetrics>()
        //     .map(GValue::from),
        // Tag::Traverser => blob.value.deserialize::<GraphSON<V2>, D, Traverser>().map(GValue::from),
        type_tag => Err(Error::Unsupported {
            tag: type_tag.to_string(),
            location: location!(),
        }),
    }
}

fn deserialize_variant<'a, D: Dialect>(value: &Value) -> Result<GValue, Error> {
    match value {
        val if is_stargraph(val) => value.deserialize::<GraphSON<V2>, D, StarGraph>().map(GValue::from),
        _ => Err(Error::Unexpected {
            expectation: "Special case".into(),
            actual: format!("{value}"),
            location: location!(),
        }),
    }
}
