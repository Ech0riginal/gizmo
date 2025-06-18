use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Float, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        let val = expect_f32!(val)?;
        Ok(Float(val))
    }
}

impl<D: Dialect> GraphsonSerializer<Float, D> for GraphSON<V3> {
    fn serialize(val: &Float) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
