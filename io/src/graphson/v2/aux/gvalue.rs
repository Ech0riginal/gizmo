use crate::graphson::Typed;
use crate::graphson::prelude::*;
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
            GValue::Barrier(val) => handle!(val, Barrier),
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
                    <Class as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Class>().map(GValue::from)
                    }
                    <Date as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Date>().map(GValue::from)
                    }
                    <Double as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Double>()
                        .map(GValue::from),
                    <Float as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Float>().map(GValue::from)
                    }
                    <Integer as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Integer>()
                        .map(GValue::from),
                    // <List as Tag_<D>>::tag => blob.value.deserialize::<Self, D, List<GValue>>().map(GValue::from),
                    <Long as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Long>().map(GValue::from)
                    }
                    // <Map as Tag_<D>>::tag => blob.value.deserialize::<Self, Map>().map(GValue::from),
                    // <Set as Tag_<D>>::tag => blob.value.deserialize::<Self, Set>().map(GValue::from),
                    <Timestamp as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Timestamp>()
                        .map(GValue::from),
                    <Uuid as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Uuid>().map(GValue::from)
                    }
                    <Edge as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Edge>().map(GValue::from)
                    }
                    <Path as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Path>().map(GValue::from)
                    }
                    <Property as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Property>()
                        .map(GValue::from),
                    <StarGraph as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, StarGraph>()
                        .map(GValue::from),
                    <TinkerGraph as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, TinkerGraph>()
                        .map(GValue::from),
                    <Tree as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Tree>().map(GValue::from)
                    }
                    <Vertex as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Vertex>()
                        .map(GValue::from),
                    <VertexProperty as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, VertexProperty>()
                        .map(GValue::from),
                    // <BulkSet as Tag_<D>>::tag => blob.value.deserialize::<Self, D, BulkSet>().map(GValue::from),
                    <Barrier as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Barrier>()
                        .map(GValue::from),
                    <Bytecode as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Bytecode>()
                        .map(GValue::from),
                    <Cardinality as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Cardinality>()
                        .map(GValue::from),
                    <Column as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Column>()
                        .map(GValue::from),
                    <Direction as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Direction>()
                        .map(GValue::from),
                    // <DT as Tag_<D>>::tag => blob.value.deserialize::<Self, DT>().map(GValue::from),
                    // <Merge as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Merge>().map(GValue::from),
                    <Metrics as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Metrics>()
                        .map(GValue::from),
                    <Order as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Order>().map(GValue::from)
                    }
                    <P as Tag_<D>>::tag => blob.value.deserialize::<Self, D, P>().map(GValue::from),
                    // <Pop as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Pop>().map(GValue::from),
                    // <Scope as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Scope>().map(GValue::from),
                    <T as Tag_<D>>::tag => blob.value.deserialize::<Self, D, T>().map(GValue::from),
                    // <TextP as Tag_<D>>::tag => blob.value.deserialize::<Self, D, TextP>().map(GValue::from),
                    <TraversalMetrics as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, TraversalMetrics>()
                        .map(GValue::from),
                    <Traverser as Tag_<D>>::tag => blob
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
