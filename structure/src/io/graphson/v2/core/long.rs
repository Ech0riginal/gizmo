use crate::io::graphson::prelude::*;

impl Deserializer<Long> for V2 {
    fn deserialize(val: &Value) -> Result<Long, Error> {
        let val = expect_i64!(val);
        Ok(Long(val))
    }
}

impl Serializer<Long> for V2 {
    fn serialize(val: &Long) -> Result<serde_json::Value, Error> {
        Ok(json!({
            "@type" : Tag::Long,
            "@value" : **val,
        }))
    }
}
