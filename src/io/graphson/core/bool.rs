use crate::io::graphson::prelude::*;

impl Deserializer<Bool> for V2 {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        let bool = get_value!(val, Value::Bool)?;
        Ok(Bool(*bool).into())
    }
}

impl Serializer<Bool> for V2 {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        Ok(Value::Bool(**val))
    }
}

passthrough!(Bool, V3 to V2);
