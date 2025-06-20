use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Integer, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Integer, Error> {
        let val = expect_i32!(val)?;
        Ok(Integer(val))
    }
}

impl<D: Dialect> GraphsonSerializer<Integer, D> for GraphSON<V2> {
    fn serialize(val: &Integer) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
