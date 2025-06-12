use crate::graphson::prelude::*;

impl Deserializer<Integer> for V3 {
    fn deserialize(val: &Value) -> Result<Integer, Error> {
        let val = expect_i32!(val).ctx::<Integer>()?;
        Ok(Integer(val))
    }
}

impl Serializer<Integer> for V3 {
    fn serialize(val: &Integer) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::Integer,
            "@value" : **val,
        }))
    }
}
