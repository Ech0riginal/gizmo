use crate::io::graphson::prelude::*;

impl Deserializer<Double> for V2 {
    fn deserialize(val: &Value) -> Result<Double, Error> {
        let val = expect_double!(val);
        Ok(Double(val))
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
