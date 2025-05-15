use crate::io::graphson::prelude::*;

impl Deserializer<Float> for V2 {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        get_value!(val, Value::Number)?
            .as_f64()
            .map(|f| Float(f as f32))
            .ok_or(Error::unexpected(val, "Not floating-point"))
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
