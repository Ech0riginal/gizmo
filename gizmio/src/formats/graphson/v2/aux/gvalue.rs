use crate::formats::graphson::prelude::*;
use crate::formats::{TypeTag, Typed};
use serde_json::Value;

impl<D: Dialect> GraphsonSerializer<GValue, D> for GraphSON<V2> {
    fn serialize(val: &GValue) -> Result<Value, Error> {
        macro_rules! serialize {
            ($val:ident, $var:ty) => {
                $val.serialize::<Self, D>()
                    .map(|v| json!({ "@type": D::tag::<$var>(), "@value": v }))
            };
        }

        match val {
            GValue::Null => Ok(Value::Null),
            GValue::Bool(val) => Ok(Value::Bool(**val)),
            GValue::String(val) => Ok(Value::String(val.to_string())),
            GValue::Class(val) => serialize!(val, Class),
            GValue::Date(val) => serialize!(val, Date),
            GValue::Double(val) => serialize!(val, Double),
            GValue::Float(val) => serialize!(val, Float),
            GValue::Integer(val) => serialize!(val, Integer),
            GValue::List(val) => val.serialize::<Self, D>(),
            GValue::Long(val) => serialize!(val, Long),
            GValue::Map(val) => val.serialize::<Self, D>(),
            GValue::Set(val) => val.serialize::<Self, D>(),
            GValue::Timestamp(val) => serialize!(val, Timestamp),
            GValue::Uuid(val) => serialize!(val, Uuid),
            GValue::Edge(val) => serialize!(val, Edge),
            GValue::Path(val) => serialize!(val, Path),
            GValue::Property(val) => serialize!(val, Property),
            GValue::StarGraph(val) => {
                let json = val
                    .serialize::<Self, D>()
                    .map(|value| json!({ D::tag::<StarGraph>(): value }))?;
                let _debug = format!("{}", &json);
                Ok(json)
            }
            GValue::TinkerGraph(val) => serialize!(val, TinkerGraph),
            GValue::Tree(val) => serialize!(val, Tree),
            GValue::Vertex(val) => serialize!(val, Vertex),
            GValue::VertexProperty(val) => serialize!(val, VertexProperty),
            GValue::Barrier(val) => serialize!(val, Barrier),
            GValue::Binding(val) => serialize!(val, Binding),
            GValue::Bytecode(val) => serialize!(val, Bytecode),
            GValue::Cardinality(val) => serialize!(val, Cardinality),
            GValue::Column(val) => serialize!(val, Column),
            GValue::Direction(val) => serialize!(val, Direction),
            GValue::Lambda(val) => serialize!(val, Lambda),
            GValue::Merge(val) => serialize!(val, Merge),
            GValue::Metrics(val) => serialize!(val, Metrics),
            GValue::Operator(val) => serialize!(val, Operator),
            GValue::Order(val) => serialize!(val, Order),
            GValue::P(val) => serialize!(val, P),
            GValue::Pick(val) => serialize!(val, Pick),
            GValue::Pop(val) => serialize!(val, Pop),
            GValue::Scope(val) => serialize!(val, Scope),
            GValue::T(val) => serialize!(val, T),
            GValue::TextP(val) => serialize!(val, TextP),
            GValue::TraversalMetrics(val) => serialize!(val, TraversalMetrics),
            GValue::Traverser(val) => serialize!(val, Traverser),
            gvalue => Err(Error::unsupported(gvalue)),
        }
    }
}

impl<D: Dialect> GraphsonDeserializer<GValue, D> for GraphSON<V2> {
    fn deserialize(value: &Value) -> Result<GValue, Error> {
        match value {
            Value::String(string) => Ok(GValue::from(string)),
            Value::Number(_) => value.deserialize::<Self, D, Integer>().map(GValue::from),
            Value::Array(_) => value
                .deserialize::<Self, D, List<GValue>>()
                .map(GValue::from),
            Value::Object(_obj) => match value.typed() {
                Ok(blob) => {
                    macro_rules! deserialize {
                        ($ty:ty) => {
                            blob.value.deserialize::<Self, D, $ty>().map(GValue::from)
                        };
                    }

                    match blob.tag {
                        TypeTag::Class => deserialize!(Class),
                        TypeTag::Date => deserialize!(Date),
                        TypeTag::Double => deserialize!(Double),
                        TypeTag::Float => deserialize!(Float),
                        TypeTag::Integer => deserialize!(Integer),
                        TypeTag::Long => deserialize!(Long),
                        TypeTag::Timestamp => deserialize!(Timestamp),
                        TypeTag::Uuid => deserialize!(Uuid),
                        TypeTag::Edge => deserialize!(Edge),
                        TypeTag::Path => deserialize!(Path),
                        TypeTag::Property => deserialize!(Property),
                        TypeTag::StarGraph => deserialize!(StarGraph),
                        TypeTag::TinkerGraph => deserialize!(TinkerGraph),
                        TypeTag::Tree => deserialize!(Tree),
                        TypeTag::Vertex => deserialize!(Vertex),
                        TypeTag::VertexProperty => deserialize!(VertexProperty),
                        TypeTag::Barrier => deserialize!(Barrier),
                        TypeTag::Binding => deserialize!(Binding),
                        TypeTag::Bytecode => deserialize!(Bytecode),
                        TypeTag::Cardinality => deserialize!(Cardinality),
                        TypeTag::Column => deserialize!(Column),
                        TypeTag::Direction => deserialize!(Direction),
                        TypeTag::Lambda => deserialize!(Lambda),
                        // TypeTag::DT => blob.value.deserialize::<Self, DT>().map(GValue::from),
                        TypeTag::Merge => deserialize!(Merge),
                        TypeTag::Metrics => deserialize!(Metrics),
                        TypeTag::Operator => deserialize!(Operator),
                        TypeTag::Order => deserialize!(Order),
                        TypeTag::P => deserialize!(P),
                        TypeTag::Pick => deserialize!(Pick),
                        TypeTag::Pop => deserialize!(Pop),
                        TypeTag::Scope => deserialize!(Scope),
                        TypeTag::T => deserialize!(T),
                        TypeTag::TextP => deserialize!(TextP),
                        TypeTag::TraversalMetrics => deserialize!(TraversalMetrics),
                        TypeTag::Traverser => deserialize!(Traverser),
                        type_tag => Err(Error::unsupported(type_tag)),
                    }
                }
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
            Value::Bool(bool) => Ok(Bool(*bool).into()),
            Value::Null => Ok(GValue::Null),
        }
    }
}

fn is_stargraph(val: &Value) -> bool {
    val.get("starVertex").is_some()
}
