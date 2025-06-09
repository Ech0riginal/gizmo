use crate::io::graphson::prelude::*;

impl Deserializer<Integer> for V2 {
    fn deserialize(val: &Value) -> Result<Integer, Error> {
        let val = expect_i32!(val);
        Ok(Integer(val))
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
