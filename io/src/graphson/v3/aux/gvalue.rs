use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonSerializer<GValue, D> for GraphSON<V3> {
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
                    <List<GValue> as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, List<GValue>>()
                        .map(GValue::from),
                    <Long as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Long>().map(GValue::from)
                    }
                    <Map<GValue, GValue> as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Map<GValue, GValue>>()
                        .map(GValue::from),
                    <Set as Tag_<D>>::tag => {
                        blob.value.deserialize::<Self, D, Set>().map(GValue::from)
                    }
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
                    <TinkerGraph as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, TinkerGraph>()
                        .map(GValue::from),
                    <Vertex as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, Vertex>()
                        .map(GValue::from),
                    <VertexProperty as Tag_<D>>::tag => blob
                        .value
                        .deserialize::<Self, D, VertexProperty>()
                        .map(GValue::from),
                    // <Barrier as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Barrier>().map(GValue::from),
                    // <Binding as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Binding>().map(GValue::from),
                    // <BulkSet as Tag_<D>>::tag => blob.value.deserialize::<Self, D, BulkSet>().map(GValue::from),
                    // <Bytecode as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Bytecode>().map(GValue::from),
                    // <Cardinality as Tag_<D>>::tag => blob
                    //     .value
                    //     .deserialize::<Self, D, Cardinality>()
                    //     .map(GValue::from),
                    // <Column as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Column>().map(GValue::from),
                    // <Direction as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Direction>().map(GValue::from),
                    // // <DT as Tag_<D>>::tag => blob.value.deserialize::<Self, D, DT>().map(GValue::from),
                    // <Merge as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Merge>().map(GValue::from),
                    // <Metrics as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Metrics>().map(GValue::from),
                    // <Operator as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Operator>().map(GValue::from),
                    // <Order as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Order>().map(GValue::from),
                    // <P as Tag_<D>>::tag => blob.value.deserialize::<Self, D, P>().map(GValue::from),
                    // <Pop as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Pop>().map(GValue::from),
                    // <Scope as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Scope>().map(GValue::from),
                    // <T as Tag_<D>>::tag => blob.value.deserialize::<Self, D, T>().map(GValue::from),
                    // <TextP as Tag_<D>>::tag => blob.value.deserialize::<Self, D, TextP>().map(GValue::from),
                    // // <TraversalExplanation as Tag_<D>>::tag => blob.value.deserialize::<Self, TraversalExplanation>().map(GValue::from),
                    // <TraversalMetrics as Tag_<D>>::tag => blob
                    //     .value
                    //     .deserialize::<Self, D, TraversalMetrics>()
                    //     .map(GValue::from),
                    // <Traverser as Tag_<D>>::tag => blob.value.deserialize::<Self, D, Traverser>().map(GValue::from),
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
