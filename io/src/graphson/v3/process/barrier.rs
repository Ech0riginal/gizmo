use crate::graphson::prelude::*;

impl Deserializer<Barrier> for V3 {
    fn deserialize(val: &Value) -> Result<Barrier, Error> {
        let str = get_value!(val, Value::String)?;
        match str.as_ref() {
            "normSack" | "norm_sack" => Ok(Barrier::NormSack),
            _ => Err(Error::Unexpected {
                actual: str.to_string(),
                expectation: "normSack".to_string(),
                location: location!(),
            }),
        }
    }
}

impl Serializer<Barrier> for V3 {
    fn serialize(val: &Barrier) -> Result<Value, Error> {
        Ok(json!({
            "@type": Tag::Barrier,
            "@value": match val {
                Barrier::NormSack => "normSack",
            }
        }))
    }
}
