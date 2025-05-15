use crate::io::graphson::prelude::*;

impl Deserializer<Double> for V2 {
    fn deserialize(val: &Value) -> Result<Double, Error> {
        get_value!(val, Value::Number)?
            .as_f64()
            .map(Double)
            .ok_or(Error::unexpected(val, "Not an f64"))
    }
}

impl Serializer<Double> for V2 {
    fn serialize(val: &Double) -> Result<Value, Error> {
        Ok(json!({
            "@type" : DOUBLE,
            "@value" : **val,
        }))
    }
}

passthrough!(Double, V3 to V2);
