use crate::graphson::prelude::*;

impl Deserializer<Float> for V2 {
    fn deserialize(val: &Value) -> Result<Float, Error> {
        let val = expect_float!(val);
        Ok(Float(val))
    }
}

impl Serializer<Float> for V2 {
    fn serialize(val: &Float) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Float,
            "@value" : **val,
        }))
    }
}
