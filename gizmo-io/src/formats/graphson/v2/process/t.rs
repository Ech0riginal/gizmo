use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<T, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<T, Error> {
        let repr = get_value!(val, Value::String)?;
        match repr.as_str() {
            T::ID => Ok(T::Id),
            T::KEY => Ok(T::Key),
            T::LABEL => Ok(T::Label),
            T::VALUE => Ok(T::Value),
            _ => Err(Error::Unexpected {
                expectation: "A valid T value".to_string(),
                actual: format!("{val}"),
                location: location!(),
            }),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<T, D> for GraphSON<V2> {
    fn serialize(val: &T) -> Result<Value, Error> {
        Ok(json!(match val {
            T::Id => T::ID,
            T::Key => T::KEY,
            T::Label => T::LABEL,
            T::Value => T::VALUE,
        }))
    }
}
