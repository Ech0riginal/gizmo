use crate::graphson::prelude::*;

impl Deserializer<Double> for V2 {
    fn deserialize(val: &Value) -> Result<Double, Error> {
        let val = expect_double!(val);
        Ok(Double(val))
    }
}

impl Serializer<Double> for V2 {
    fn serialize(val: &Double) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Double,
            "@value" : **val,
        }))
    }
}

