use crate::graphson::prelude::*;

impl Deserializer<Class> for V3 {
    fn deserialize(val: &Value) -> Result<Class, Error> {
        let class = get_value!(val, Value::String).ctx::<Class>()?;
        Ok(class.into())
    }
}

impl Serializer<Class> for V3 {
    fn serialize(val: &Class) -> Result<Value, Error> {
        Ok(json!({
            "@type" : Tag::Class,
            "@value" : **val,
        }))
    }
}
