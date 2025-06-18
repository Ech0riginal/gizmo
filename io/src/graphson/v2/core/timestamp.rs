use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Timestamp, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Timestamp, Error> {
        let val = expect_i64!(val)?;
        let ms_since_epoch = Timestamp(val);
        Ok(ms_since_epoch)
    }
}

impl<D: Dialect> GraphsonSerializer<Timestamp, D> for GraphSON<V2> {
    fn serialize(val: &Timestamp) -> Result<Value, Error> {
        Ok(json!(val.0))
    }
}
