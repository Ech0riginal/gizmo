use crate::io::graphson::prelude::*;

impl Deserializer<T> for V2 {
    fn deserialize(val: &Value) -> Result<T, Error> {
        let string = get_value!(val, Value::String)?;
        let t = match string.as_str() {
            "id" => T::Id,
            "key" => T::Key,
            "label" => T::Label,
            "value" => T::Value,
            _ => {
                return Err(Error::UnexpectedJson {
                    msg: "A valid T value was expected".to_string(),
                    value: val.clone(),
                });
            }
        };
        Ok(t)
    }
}

impl Serializer<T> for V2 {
    fn serialize(val: &T) -> Result<serde_json::Value, Error> {
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
