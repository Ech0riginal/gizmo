use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Double, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Double, Error> {
        let val = expect_f64!(val)?;
        Ok(Double(val))
    }
}

impl<D: Dialect> GraphsonSerializer<Double, D> for GraphSON<V2> {
    fn serialize(val: &Double) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
