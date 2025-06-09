use crate::io::graphson::prelude::*;

impl Deserializer<Class> for V2 {
    fn deserialize(val: &Value) -> Result<Class, Error> {
        let class = get_value!(val, Value::String)?;
        Ok(class.into())
    }
}

impl Serializer<Class> for V2 {
    fn serialize(val: &Class) -> Result<Value, Error> {
        Ok(json!({
            "@type" : CLASS,
            "@value" : **val,
        }))
    }
}

passthrough!(Class, V3 to V2);
