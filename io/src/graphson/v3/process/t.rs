use crate::graphson::prelude::*;

impl Deserializer<T> for V3 {
    fn deserialize(val: &Value) -> Result<T, Error> {
        let string = get_value!(val, Value::String)?;
        let t = match string.as_str() {
            "id" => T::Id,
            "key" => T::Key,
            "label" => T::Label,
            "value" => T::Value,
            _ => {
                return Err(Error::Unexpected {
                    expectation: "A valid T value".to_string(),
                    actual: format!("{val:?}"),
                    location: location!(),
                });
            }
        };
        Ok(t)
    }
}

impl Serializer<T> for V3 {
    fn serialize(val: &T) -> Result<Value, Error> {
        Ok(json!({
            "@type": Tag::T,
            "@value": match val {
                T::Id => "id",
                T::Key => "key",
                T::Label => "label",
                T::Value => "value",
            }
        }))
    }
}
