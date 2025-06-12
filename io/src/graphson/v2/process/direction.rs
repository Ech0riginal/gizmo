use crate::graphson::prelude::*;

impl Deserializer<Direction> for V2 {
    fn deserialize(val: &Value) -> Result<Direction, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_str() {
            "OUT" => Ok(Direction::Out),
            "IN" => Ok(Direction::In),
            _ => Err(Error::Unexpected {
                expectation: "de-wonked json".to_string(),
                actual: format!("{val}"),
                location: location!(),
            }),
        }
    }
}

impl Serializer<Direction> for V2 {
    fn serialize(val: &Direction) -> Result<Value, Error> {
        let direction_str = match val {
            Direction::Out | Direction::From => "OUT",
            Direction::In | Direction::To => "IN",
        };
        Ok(json!({
            "@type" : Tag::Direction,
            "@value" : direction_str,
        }))
    }
}
