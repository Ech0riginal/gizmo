use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Class, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Class, Error> {
        let class = get_value!(val, Value::String)?;
        Ok(class.into())
    }
}

impl<D: Dialect> GraphsonSerializer<Class, D> for GraphSON<V2> {
    fn serialize(val: &Class) -> Result<Value, Error> {
        Ok(json!(**val))
    }
}
