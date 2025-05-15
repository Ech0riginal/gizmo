use crate::io::graphson::prelude::*;

impl Deserializer<Float> for V2 {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        let val = expect_float!(val);
        Ok(Float(val))
    }
}

impl Serializer<Float> for V2 {
    fn serialize(val: &Float) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : FLOAT,
            "@value" : **val,
        }))
    }
}

passthrough!(Float, V3 to V2);
