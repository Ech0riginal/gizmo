use crate::graphson::prelude::*;

impl Deserializer<Float> for V3 {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        let val = expect_f32!(val).ctx::<Float>()?;
        Ok(Float(val))
    }
}

impl Serializer<Float> for V3 {
    fn serialize(val: &Float) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::Float,
            "@value" : **val,
        }))
    }
}
