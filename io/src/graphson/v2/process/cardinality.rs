use crate::graphson::prelude::*;

impl Deserializer<Cardinality> for V2 {
    fn deserialize(val: &Value) -> Result<Cardinality, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_str() {
            "list" => Ok(Cardinality::List),
            "set" => Ok(Cardinality::Set),
            "single" => Ok(Cardinality::Single),
            _ => Err(Error::Unexpected {
                actual: string.to_string(),
                expectation: "'list', 'set', or 'single'".to_string(),
                location: location!(),
            }),
        }
    }
}

impl Serializer<Cardinality> for V2 {
    fn serialize(val: &Cardinality) -> Result<Value, Error> {
        let str = match val {
            Cardinality::List => "list",
            Cardinality::Set => "set",
            Cardinality::Single => "single",
        };
        Ok(json!({
            "@type": Tag::Cardinality,
            "@value": str,
        }))
    }
}
