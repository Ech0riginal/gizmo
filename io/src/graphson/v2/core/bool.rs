use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Bool, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        let bool = get_value!(val, Value::Bool)?;
        Ok(Bool(*bool))
    }
}

impl<D: Dialect> GraphsonSerializer<Bool, D> for GraphSON<V2> {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        Ok(Value::Bool(**val))
    }
}
