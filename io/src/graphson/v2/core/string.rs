use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<String, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<String, Error> {
        let string = get_value!(val, Value::String)?;
        Ok(string.clone())
    }
}

impl<D: Dialect> GraphsonSerializer<String, D> for GraphSON<V2> {
    fn serialize(val: &String) -> Result<Value, Error> {
        Ok(json!(val))
    }
}
