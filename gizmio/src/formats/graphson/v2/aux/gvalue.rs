use serde_json::Value;

use crate::formats::Typed;
use crate::formats::graphson::prelude::*;

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
            GValue::StarGraph(val) => val
                .serialize::<Self, D>()
                .map(|value| json!({ D::tag::<StarGraph>(): value })),
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
                        <Class as AST<D>>::tag => deserialize!(Class),
                        <Date as AST<D>>::tag => deserialize!(Date),
                        <Double as AST<D>>::tag => deserialize!(Double),
                        <Float as AST<D>>::tag => deserialize!(Float),
                        <Integer as AST<D>>::tag => deserialize!(Integer),
                        // <List as AST<D>>::tag => deserialize!(List<GValue>),,
                        <Long as AST<D>>::tag => deserialize!(Long),
                        // <Map as AST<D>>::tag => blob.value.deserialize::<Self, Map>().map(GValue::from),
                        // <Set as AST<D>>::tag => blob.value.deserialize::<Self, Set>().map(GValue::from),
                        <Timestamp as AST<D>>::tag => deserialize!(Timestamp),
                        <Uuid as AST<D>>::tag => deserialize!(Uuid),
                        <Edge as AST<D>>::tag => deserialize!(Edge),
                        <Path as AST<D>>::tag => deserialize!(Path),
                        <Property as AST<D>>::tag => deserialize!(Property),
                        <StarGraph as AST<D>>::tag => deserialize!(StarGraph),
                        <TinkerGraph as AST<D>>::tag => deserialize!(TinkerGraph),
                        <Tree as AST<D>>::tag => deserialize!(Tree),
                        <Vertex as AST<D>>::tag => deserialize!(Vertex),
                        <VertexProperty as AST<D>>::tag => deserialize!(VertexProperty),
                        // <BulkSet as AST<D>>::tag => deserialize!(BulkSet),,
                        <Barrier as AST<D>>::tag => deserialize!(Barrier),
                        <Binding as AST<D>>::tag => deserialize!(Binding),
                        <Bytecode as AST<D>>::tag => deserialize!(Bytecode),
                        <Cardinality as AST<D>>::tag => deserialize!(Cardinality),
                        <Column as AST<D>>::tag => deserialize!(Column),
                        <Direction as AST<D>>::tag => deserialize!(Direction),
                        <Lambda as AST<D>>::tag => deserialize!(Lambda),
                        // <DT as AST<D>>::tag => blob.value.deserialize::<Self, DT>().map(GValue::from),
                        <Merge as AST<D>>::tag => deserialize!(Merge),
                        <Metrics as AST<D>>::tag => deserialize!(Metrics),
                        <Operator as AST<D>>::tag => deserialize!(Operator),
                        <Order as AST<D>>::tag => deserialize!(Order),
                        <P as AST<D>>::tag => deserialize!(P),
                        <Pick as AST<D>>::tag => deserialize!(Pick),
                        <Pop as AST<D>>::tag => deserialize!(Pop),
                        <Scope as AST<D>>::tag => deserialize!(Scope),
                        <T as AST<D>>::tag => deserialize!(T),
                        <TextP as AST<D>>::tag => deserialize!(TextP),
                        <TraversalMetrics as AST<D>>::tag => deserialize!(TraversalMetrics),
                        <Traverser as AST<D>>::tag => deserialize!(Traverser),
                        type_tag => Err(Error::Unsupported {
                            tag: type_tag.to_string(),
                            location: location!(),
                        }),
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
