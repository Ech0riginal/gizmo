use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Long, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Long, Error> {
        let val = expect_i64!(val)?;
        Ok(Long(val))
    }
}

impl<D: Dialect> GraphsonSerializer<Long, D> for GraphSON<V3> {
    fn serialize(val: &Long) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
