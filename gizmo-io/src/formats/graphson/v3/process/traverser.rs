use crate::formats::graphson::prelude::*;

const BULK: &str = "bulk";
const VALUE: &str = "value";

impl<D: Dialect> GraphsonDeserializer<Traverser, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Traverser, Error> {
        let map = get_value!(val, Value::Object)?;
        let bulk = map
            .ensure(BULK)?
            .deserialize::<Self, D, GValue>()
            .map(Long::from)?;
        let value = map.ensure(VALUE)?.deserialize::<Self, D, GValue>()?.boxed();
        Ok(Traverser { bulk, value })
    }
}
impl<D: Dialect> GraphsonSerializer<Traverser, D> for GraphSON<V3> {
    fn serialize(val: &Traverser) -> Result<Value, Error> {
        Ok(json!({
            BULK: val.bulk.gvalue().serialize::<Self, D>()?,
            VALUE: val.value.serialize::<Self, D>()?,
        }))
    }
}
