use crate::graphson::prelude::*;

impl Deserializer<Cardinality> for V2 {
    fn deserialize(val: &Value) -> Result<Cardinality, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_str() {
            "list" => Ok(Cardinality::List),
            "set" => Ok(Cardinality::Set),
            "single" => Ok(Cardinality::Single),
            _ => Err(Error::UnexpectedJson {
                msg: "".into(),
                value: val.clone(),
            }),
        }
    }
}

impl Serializer<Cardinality> for V2 {
    fn serialize(val: &Cardinality) -> Result<Value, Leaf> {
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
