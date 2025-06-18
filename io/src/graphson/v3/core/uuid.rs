use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Uuid, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Uuid, Error> {
        let string = get_value!(val, Value::String)?;
        let uuid = Uuid::parse_str(string).map_err(|_| Error::Unexpected {
            expectation: "A valid Uuid".to_string(),
            actual: format!("{:?}", val.clone()),
            location: location!(),
        })?;
        Ok(uuid)
    }
}

impl<D: Dialect> GraphsonSerializer<Uuid, D> for GraphSON<V3> {
    fn serialize(val: &Uuid) -> Result<Value, Error> {
        Ok(json!(val.to_string()))
    }
}
