use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Float, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        let val = expect_f32!(val)?;
        Ok(Float(val))
    }
}

impl<D: Dialect> GraphsonSerializer<Float, D> for GraphSON<V2> {
    fn serialize(val: &Float) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
