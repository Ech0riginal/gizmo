use crate::io::graphson::prelude::*;

impl Deserializer<Uuid> for V2 {
    fn deserialize(val: &Value) -> Result<Uuid, Error> {
        let val = get_value!(val, Value::String)?;
        let uuid = uuid::Uuid::parse_str(&val)?;
        Ok(uuid)
    }
}

impl Serializer<Uuid> for V2 {
    fn serialize(val: &Uuid) -> Result<Value, Error> {
        Ok(json!({
            "@type" : UUID,
            "@value" : val.to_string()
        }))
    }
}

passthrough!(Uuid, V3 to V2);
