use crate::graphson::prelude::*;
impl Deserializer<Bool> for V3 {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        let bool = get_value!(val, Value::Bool).ctx::<Bool>()?;
        Ok(Bool(*bool))
    }
}

impl Serializer<Bool> for V3 {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        Ok(Value::Bool(**val))
    }
}
