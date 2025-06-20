use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Uuid, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Uuid, Error> {
        let val = get_value!(val, Value::String)?;
        let uuid = uuid::Uuid::parse_str(val)?;
        Ok(uuid)
    }
}

impl<D: Dialect> GraphsonSerializer<Uuid, D> for GraphSON<V2> {
    fn serialize(val: &Uuid) -> Result<Value, Error> {
        Ok(json!(val.to_string()))
    }
}
