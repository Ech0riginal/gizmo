use crate::graphson::prelude::*;

impl Deserializer<Integer> for V2 {
    fn deserialize(val: &Value) -> Result<Integer, Error> {
        let val = expect_i32!(val);
        Ok(Integer(val))
    }
}

impl Serializer<Integer> for V2 {
    fn serialize(val: &Integer) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Integer,
            "@value" : **val,
        }))
    }
}
