use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Double, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Double, Error> {
        let val = expect_f64!(val)?;
        Ok(Double(val))
    }
}

impl<D: Dialect> GraphsonSerializer<Double, D> for GraphSON<V3> {
    fn serialize(val: &Double) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
