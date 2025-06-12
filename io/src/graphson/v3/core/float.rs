use crate::graphson::prelude::*;

impl Deserializer<Float> for V3 {
    fn deserialize(val: &Value) -> Result<Float, Leaf> {
        let val = expect_f32!(val).ctx::<Float>()?;
        Ok(Float(val))
    }
}

impl Serializer<Float> for V3 {
    fn serialize(val: &Float) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Float,
            "@value" : **val,
        }))
    }
}
