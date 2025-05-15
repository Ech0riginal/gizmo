use crate::io::graphson::prelude::*;

impl Deserializer<Integer> for V2 {
    fn deserialize(val: &Value) -> Result<Integer, Error> {
        get_value!(val, Value::Number)?
            .as_i64()
            .map(|i| Integer(i as i32))
            .ok_or(Error::unexpected(val, "Not a number"))
    }
}

impl Serializer<Integer> for V2 {
    fn serialize(val: &Integer) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : INT,
            "@value" : **val,
        }))
    }
}

passthrough!(Integer, V3 to V2);
