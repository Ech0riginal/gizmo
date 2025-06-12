use crate::graphson::prelude::*;

impl Deserializer<String> for V2 {
    fn deserialize(val: &Value) -> Result<String, Error> {
        let string = get_value!(val, Value::String)?;
        Ok(string.clone())
    }
}

impl Serializer<String> for V2 {
    fn serialize(val: &String) -> Result<Value, Leaf> {
        Ok(json!(val))
    }
}
