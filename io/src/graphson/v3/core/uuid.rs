use crate::graphson::prelude::*;

impl Deserializer<Uuid> for V3 {
    fn deserialize(val: &Value) -> Result<Uuid, Leaf> {
        let string = get_value!(val, Value::String).ctx::<Uuid>()?;
        let uuid = Uuid::parse_str(string)
            .map_err(|_| Leaf::Unexpected {
                expectation: "A valid Uuid".to_string(),
                actual: format!("{:?}", val.clone()),
                location: location!(),
            })
            .ctx::<Uuid>()?;
        Ok(uuid)
    }
}

impl Serializer<Uuid> for V3 {
    fn serialize(val: &Uuid) -> Result<Value, Leaf> {
        Ok(json!({
            "@type" : Tag::Uuid,
            "@value" : val.to_string()
        }))
    }
}
