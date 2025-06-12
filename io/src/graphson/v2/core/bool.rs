use crate::graphson::prelude::*;

impl Deserializer<Bool> for V2 {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        let bool = get_value!(val, Value::Bool)?;
        Ok(Bool(*bool))
    }
}

impl Serializer<Bool> for V2 {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        Ok(Value::Bool(**val))
    }
}
