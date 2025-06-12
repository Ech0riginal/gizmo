use crate::graphson::prelude::*;

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
            GValue::TinkerGraph(v) => v.serialize::<Self>(),
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
            gvalue => Err(Error::Unexpected {
                expectation: "a supported GValue".to_string(),
                actual: format!("{gvalue}"),
                location: location!(),
            }),
        }
    }
}

impl Deserializer<GValue> for V3 {
    fn deserialize(val: &Value) -> Result<GValue, Error> {
        match val {
            Value::String(string) => Ok(GValue::from(string)),
            Value::Number(_) => val
                .deserialize::<Self, Integer>()
                .ctx::<GValue>()
                .map(GValue::from),
            Value::Object(_obj) => match val.typed() {
                Ok(blob) => deserialize(blob).ctx::<GValue>(),
                Err(err) => Err(err),
            },
            Value::Array(values) => {
                let collection = values
                    .iter()
                    .map(Self::deserialize)
                    .collect::<Result<Vec<_>, Error>>()
                    .ctx::<GValue>()?
                    .into();
                Ok(GValue::List(collection))
            }
            Value::Bool(bool) => Ok(Bool(*bool).into()),
            Value::Null => Ok(GValue::Null),
        }
    }
}

fn deserialize<'a>(blob: Type<'a>) -> Result<GValue, Error> {
    match blob.tag {
        Tag::Class => blob.value.deserialize::<V3, Class>().map(GValue::from),
        Tag::Date => blob.value.deserialize::<V3, Date>().map(GValue::from),
        Tag::Double => blob.value.deserialize::<V3, Double>().map(GValue::from),
        Tag::Float => blob.value.deserialize::<V3, Float>().map(GValue::from),
        Tag::Integer => blob.value.deserialize::<V3, Integer>().map(GValue::from),
        Tag::List => blob
            .value
            .deserialize::<V3, List<GValue>>()
            .map(GValue::from),
        Tag::Long => blob.value.deserialize::<V3, Long>().map(GValue::from),
        Tag::Map => blob
            .value
            .deserialize::<V3, Map<GValue, GValue>>()
            .map(GValue::from),
        Tag::Set => blob.value.deserialize::<V3, Set>().map(GValue::from),
        Tag::Timestamp => blob.value.deserialize::<V3, Timestamp>().map(GValue::from),
        Tag::Uuid => blob.value.deserialize::<V3, Uuid>().map(GValue::from),
        Tag::Edge => blob.value.deserialize::<V3, Edge>().map(GValue::from),
        Tag::Path => blob.value.deserialize::<V3, Path>().map(GValue::from),
        Tag::Property => blob.value.deserialize::<V3, Property>().map(GValue::from),
        Tag::TinkerGraph => blob
            .value
            .deserialize::<V3, TinkerGraph>()
            .map(GValue::from),
        Tag::Vertex => blob.value.deserialize::<V3, Vertex>().map(GValue::from),
        Tag::VertexProperty => blob
            .value
            .deserialize::<V3, VertexProperty>()
            .map(GValue::from),
        Tag::BulkSet => blob.value.deserialize::<V3, BulkSet>().map(GValue::from),
        // Tag::Bytecode => blob.value.deserialize::<V3, Bytecode>().map(GValue::from),
        // Tag::Cardinality => blob.value.deserialize::<Self, Cardinality>().map(GValue::from),
        // Tag::Column => blob.value.deserialize::<Self, Column>().map(GValue::from),
        // Tag::Direction => blob.value.deserialize::<Self, Direction>().map(GValue::from),
        // Tag::DT => blob.value.deserialize::<Self, DT>().map(GValue::from),
        // Tag::Merge => blob.value.deserialize::<Self, Merge>().map(GValue::from),
        Tag::Metrics => blob.value.deserialize::<V3, Metrics>().map(GValue::from),
        // Tag::Order => blob.value.deserialize::<Self, Order>().map(GValue::from),
        // Tag::P => blob.value.deserialize::<Self, P>().map(GValue::from),
        // Tag::Pop => blob.value.deserialize::<Self, Pop>().map(GValue::from),
        // Tag::Scope => blob.value.deserialize::<Self, Scope>().map(GValue::from),
        // Tag::T => blob.value.deserialize::<Self, T>().map(GValue::from),
        // Tag::TextP => blob.value.deserialize::<Self, TextP>().map(GValue::from),
        // Tag::TraversalExplanation => blob.value.deserialize::<Self, TraversalExplanation>().map(GValue::from),
        Tag::TraversalMetrics => blob
            .value
            .deserialize::<V3, TraversalMetrics>()
            .map(GValue::from),
        // Tag::Traverser => blob.value.deserialize::<Self, Traverser>().map(GValue::from),
        type_tag => Err(Error::Unsupported {
            tag: type_tag.to_string(),
            location: location!(),
        }),
    }
}
