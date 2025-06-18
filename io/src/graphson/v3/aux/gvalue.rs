use crate::graphson::prelude::*;
use crate::graphson::tags::{Tag, Typed};

impl<D: Dialect> GraphsonSerializer<GValue, D> for GraphSON<V3> {
    fn serialize(val: &GValue) -> Result<Value, Error> {
        macro_rules! handle {
            ($val:ident, $var:ty) => {
                Ok(json!({
                    "@type": $val.serialize::<Self, D>()?,
                    "@value": D::tag::<$var>(),
                }))
            };
        }

        match val {
            GValue::Null => Ok(Value::Null),
            GValue::Bool(val) => handle!(val, Bool),
            GValue::Class(val) => handle!(val, Class),
            GValue::Date(val) => handle!(val, Date),
            GValue::Double(val) => handle!(val, Double),
            GValue::Float(val) => handle!(val, Float),
            GValue::Integer(val) => handle!(val, Integer),
            GValue::List(val) => handle!(val, List<GValue>),
            GValue::Long(val) => handle!(val, Long),
            GValue::Map(val) => handle!(val, Map<GValue, GValue>),
            GValue::Set(val) => handle!(val, Set),
            GValue::String(val) => Ok(json!(val)),
            GValue::Timestamp(val) => handle!(val, Timestamp),
            GValue::Uuid(val) => handle!(val, Uuid),
            GValue::Edge(val) => handle!(val, Edge),
            GValue::Path(val) => handle!(val, Path),
            GValue::Property(val) => handle!(val, Property),
            GValue::TinkerGraph(val) => handle!(val, TinkerGraph),
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
            GValue::TextP(val) => handle!(val, TextP),
            GValue::TraversalMetrics(val) => handle!(val, TraversalMetrics),
            GValue::Traverser(val) => handle!(val, Traverser),
            GValue::Metrics(val) => handle!(val, Metrics),
            GValue::Geometry(val) => handle!(val, Geometry),
            GValue::Merge(val) => handle!(val, Merge),
            GValue::BulkSet(val) => handle!(val, BulkSet),
            gvalue => Err(Error::Unexpected {
                expectation: "a supported GValue".to_string(),
                actual: format!("{gvalue}"),
                location: location!(),
            }),
        }
    }
}

impl<D: Dialect> GraphsonDeserializer<GValue, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<GValue, Error> {
        match val {
            Value::String(string) => Ok(GValue::from(string)),
            Value::Number(_) => val.deserialize::<Self, D, Integer>().map(GValue::from),
            Value::Object(_obj) => match val.typed() {
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
                    Tag::List => blob
                        .value
                        .deserialize::<Self, D, List<GValue>>()
                        .map(GValue::from),
                    Tag::Long => blob.value.deserialize::<Self, D, Long>().map(GValue::from),
                    Tag::Map => blob
                        .value
                        .deserialize::<Self, D, Map<GValue, GValue>>()
                        .map(GValue::from),
                    Tag::Set => blob.value.deserialize::<Self, D, Set>().map(GValue::from),
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
                    Tag::TinkerGraph => blob
                        .value
                        .deserialize::<Self, D, TinkerGraph>()
                        .map(GValue::from),
                    Tag::Vertex => blob
                        .value
                        .deserialize::<Self, D, Vertex>()
                        .map(GValue::from),
                    Tag::VertexProperty => blob
                        .value
                        .deserialize::<Self, D, VertexProperty>()
                        .map(GValue::from),
                    // Tag::Barrier => blob.value.deserialize::<Self, D, Barrier>().map(GValue::from),
                    // Tag::Binding => blob.value.deserialize::<Self, D, Binding>().map(GValue::from),
                    // Tag::BulkSet => blob.value.deserialize::<Self, D, BulkSet>().map(GValue::from),
                    // Tag::Bytecode => blob.value.deserialize::<Self, D, Bytecode>().map(GValue::from),
                    // Tag::Cardinality => blob
                    //     .value
                    //     .deserialize::<Self, D, Cardinality>()
                    //     .map(GValue::from),
                    // Tag::Column => blob.value.deserialize::<Self, D, Column>().map(GValue::from),
                    // Tag::Direction => blob.value.deserialize::<Self, D, Direction>().map(GValue::from),
                    // // Tag::DT => blob.value.deserialize::<Self, D, DT>().map(GValue::from),
                    // Tag::Merge => blob.value.deserialize::<Self, D, Merge>().map(GValue::from),
                    // Tag::Metrics => blob.value.deserialize::<Self, D, Metrics>().map(GValue::from),
                    // Tag::Operator => blob.value.deserialize::<Self, D, Operator>().map(GValue::from),
                    // Tag::Order => blob.value.deserialize::<Self, D, Order>().map(GValue::from),
                    // Tag::P => blob.value.deserialize::<Self, D, P>().map(GValue::from),
                    // Tag::Pop => blob.value.deserialize::<Self, D, Pop>().map(GValue::from),
                    // Tag::Scope => blob.value.deserialize::<Self, D, Scope>().map(GValue::from),
                    // Tag::T => blob.value.deserialize::<Self, D, T>().map(GValue::from),
                    // Tag::TextP => blob.value.deserialize::<Self, D, TextP>().map(GValue::from),
                    // // Tag::TraversalExplanation => blob.value.deserialize::<Self, TraversalExplanation>().map(GValue::from),
                    // Tag::TraversalMetrics => blob
                    //     .value
                    //     .deserialize::<Self, D, TraversalMetrics>()
                    //     .map(GValue::from),
                    // Tag::Traverser => blob.value.deserialize::<Self, D, Traverser>().map(GValue::from),
                    type_tag => Err(Error::Unsupported {
                        tag: type_tag.to_string(),
                        location: location!(),
                    }),
                },
                Err(err) => Err(err),
            },
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(|v| v.deserialize::<Self, D, GValue>())
                    .collect::<Result<List<_>, _>>()?;
                Ok(GValue::List(collection))
            }
            Value::Bool(bool) => Ok(Bool(*bool).into()),
            Value::Null => Ok(GValue::Null),
        }
    }
}
