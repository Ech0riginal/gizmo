use crate::graphson::prelude::*;

impl Deserializer<Timestamp> for V3 {
    fn deserialize(val: &Value) -> Result<Timestamp, Error> {
        let val = expect_i64!(val)?;
        let ms_since_epoch = Timestamp(val);
        Ok(ms_since_epoch)
    }
}

impl Serializer<Timestamp> for V3 {
    fn serialize(val: &Timestamp) -> Result<Value, Error> {
        Ok(json!({
            "@type": Tag::Timestamp,
            "@value": val.0,
        }))
    }
}
