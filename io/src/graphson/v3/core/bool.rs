use crate::graphson::prelude::*;
impl<D: Dialect> GraphsonDeserializer<Bool, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        let bool = get_value!(val, Value::Bool)?;
        Ok(Bool(*bool))
    }
}

impl<D: Dialect> GraphsonSerializer<Bool, D> for GraphSON<V3> {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
