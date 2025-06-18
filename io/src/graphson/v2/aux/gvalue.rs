use crate::graphson::prelude::*;
use crate::graphson::tags::{Tag, Typed};
use serde_json::Value;

impl<D: Dialect> GraphsonSerializer<GValue, D> for GraphSON<V2> {
    fn serialize(val: &GValue) -> Result<Value, Error> {
        macro_rules! handle {
            ($val:ident, $var:ty) => {
                $val.serialize::<Self, D>()
                    .map(|v| json!({ "@type": D::tag::<$var>(), "@value": v }))
            };
        }
        match val {
            GValue::Null => Ok(Value::Null),
            GValue::Bool(val) => Ok(Value::Bool(**val)),
            GValue::String(val) => Ok(Value::String(val.to_string())),
            GValue::Class(val) => handle!(val, Class),
            GValue::Date(val) => handle!(val, Date),
            GValue::Double(val) => handle!(val, Double),
            GValue::Float(val) => handle!(val, Float),
            GValue::Integer(val) => handle!(val, Integer),
            GValue::List(val) => val.serialize::<Self, D>(),
            GValue::Long(val) => handle!(val, Long),
            GValue::Map(val) => val.serialize::<Self, D>(),
            GValue::Set(val) => val.serialize::<Self, D>(),
            GValue::Timestamp(val) => handle!(val, Timestamp),
            GValue::Uuid(val) => handle!(val, Uuid),
            GValue::Edge(val) => handle!(val, Edge),
            GValue::Path(val) => handle!(val, Path),
            GValue::Property(val) => handle!(val, Property),
            GValue::StarGraph(val) => val
                .serialize::<Self, D>()
                .map(|value| json!({ D::tag::<StarGraph>(): value })),
            GValue::TinkerGraph(val) => handle!(val, TinkerGraph),
            GValue::Tree(val) => handle!(val, Tree),
            GValue::Vertex(val) => handle!(val, Vertex),
            GValue::VertexProperty(val) => handle!(val, VertexProperty),
            GValue::Bytecode(val) => handle!(val, Bytecode),
            GValue::Cardinality(val) => handle!(val, Cardinality),
            GValue::Column(val) => handle!(val, Column),
            GValue::Direction(val) => handle!(val, Direction),
            GValue::Order(val) => handle!(val, Order),
            GValue::Pop(val) => handle!(val, Pop),
            GValue::P(val) => handle!(val, P),
            GValue::Scope(val) => handle!(val, Scope),
            GValue::T(val) => handle!(val, T),
            GValue::TraversalMetrics(val) => handle!(val, TraversalMetrics),
            GValue::Traverser(val) => handle!(val, Traverser),
            GValue::Metrics(val) => handle!(val, Metrics),
            GValue::TextP(val) => handle!(val, TextP),
            value => Err(Error::Unsupported {
                tag: format!("{value}"),
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
                Ok(blob) => match blob.tag {
                    Tag::Class => blob.value.deserialize::<Self, D, Class>().map(GValue::from),
                    Tag::Date => blob.value.deserialize::<Self, D, Date>().map(GValue::from),
                    Tag::Double => blob
                        .value
                        .deserialize::<Self, D, Double>()
                        .map(GValue::from),
                    Tag::Float => blob.value.deserialize::<Self, D, Float>().map(GValue::from),
                    Tag::Integer => blob
                        .value
                        .deserialize::<Self, D, Integer>()
                        .map(GValue::from),
                    // Tag::List => blob.value.deserialize::<Self, D, List<GValue>>().map(GValue::from),
                    Tag::Long => blob.value.deserialize::<Self, D, Long>().map(GValue::from),
                    // Tag::Map => blob.value.deserialize::<Self, Map>().map(GValue::from),
                    // Tag::Set => blob.value.deserialize::<Self, Set>().map(GValue::from),
                    Tag::Timestamp => blob
                        .value
                        .deserialize::<Self, D, Timestamp>()
                        .map(GValue::from),
                    Tag::Uuid => blob.value.deserialize::<Self, D, Uuid>().map(GValue::from),
                    Tag::Edge => blob.value.deserialize::<Self, D, Edge>().map(GValue::from),
                    Tag::Path => blob.value.deserialize::<Self, D, Path>().map(GValue::from),
                    Tag::Property => blob
                        .value
                        .deserialize::<Self, D, Property>()
                        .map(GValue::from),
                    Tag::StarGraph => blob
                        .value
                        .deserialize::<Self, D, StarGraph>()
                        .map(GValue::from),
                    Tag::TinkerGraph => blob
                        .value
                        .deserialize::<Self, D, TinkerGraph>()
                        .map(GValue::from),
                    Tag::Tree => blob.value.deserialize::<Self, D, Tree>().map(GValue::from),
                    Tag::Vertex => blob
                        .value
                        .deserialize::<Self, D, Vertex>()
                        .map(GValue::from),
                    Tag::VertexProperty => blob
                        .value
                        .deserialize::<Self, D, VertexProperty>()
                        .map(GValue::from),
                    // Tag::BulkSet => blob.value.deserialize::<Self, D, BulkSet>().map(GValue::from),
                    Tag::Bytecode => blob
                        .value
                        .deserialize::<Self, D, Bytecode>()
                        .map(GValue::from),
                    Tag::Cardinality => blob
                        .value
                        .deserialize::<Self, D, Cardinality>()
                        .map(GValue::from),
                    Tag::Column => blob
                        .value
                        .deserialize::<Self, D, Column>()
                        .map(GValue::from),
                    Tag::Direction => blob
                        .value
                        .deserialize::<Self, D, Direction>()
                        .map(GValue::from),
                    // Tag::DT => blob.value.deserialize::<Self, DT>().map(GValue::from),
                    // Tag::Merge => blob.value.deserialize::<Self, D, Merge>().map(GValue::from),
                    Tag::Metrics => blob
                        .value
                        .deserialize::<Self, D, Metrics>()
                        .map(GValue::from),
                    Tag::Order => blob.value.deserialize::<Self, D, Order>().map(GValue::from),
                    Tag::P => blob.value.deserialize::<Self, D, P>().map(GValue::from),
                    // Tag::Pop => blob.value.deserialize::<Self, D, Pop>().map(GValue::from),
                    // Tag::Scope => blob.value.deserialize::<Self, D, Scope>().map(GValue::from),
                    Tag::T => blob.value.deserialize::<Self, D, T>().map(GValue::from),
                    // Tag::TextP => blob.value.deserialize::<Self, D, TextP>().map(GValue::from),
                    Tag::TraversalMetrics => blob
                        .value
                        .deserialize::<Self, D, TraversalMetrics>()
                        .map(GValue::from),
                    Tag::Traverser => blob
                        .value
                        .deserialize::<Self, D, Traverser>()
                        .map(GValue::from),
                    type_tag => Err(Error::Unsupported {
                        tag: type_tag.to_string(),
                        location: location!(),
                    }),
                },
                Err(err) => match err {
                    Error::Missing { .. } => match value {
                        val if is_stargraph(val) => value
                            .deserialize::<GraphSON<V2>, D, StarGraph>()
                            .map(GValue::from),
                        _ => Err(Error::Unexpected {
                            expectation: "Special case".into(),
                            actual: format!("{value}"),
                            location: location!(),
                        }),
                    },
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
