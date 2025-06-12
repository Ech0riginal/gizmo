use crate::graphson::prelude::*;

impl Deserializer<String> for V3 {
    fn deserialize(val: &Value) -> Result<String, Leaf> {
        let string = get_value!(val, Value::String).ctx::<String>()?;
        Ok(string.clone())
    }
}

impl Serializer<String> for V3 {
    fn serialize(val: &String) -> Result<Value, Leaf> {
        Ok(json!(val))
    }
}

impl<'a> Serializer<&'a str> for V3 {
    fn serialize(val: &&'a str) -> Result<Value, Leaf> {
        Ok(json!(val))
    }
}
