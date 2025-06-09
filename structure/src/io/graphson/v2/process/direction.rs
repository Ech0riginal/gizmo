use crate::io::graphson::prelude::*;

impl Deserializer<Direction> for V2 {
    fn deserialize(val: &Value) -> Result<Direction, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_str() {
            "OUT" => Ok(Direction::Out),
            "IN" => Ok(Direction::In),
            _ => Err(Error::UnexpectedJson {
                msg: "Json's wonky.".to_string(),
                value: val.clone(),
            }),
        }
    }
}

impl Serializer<Direction> for V2 {
    fn serialize(val: &Direction) -> Result<serde_json::Value, Error> {
        let direction_str = match val {
            Direction::Out | Direction::From => "OUT",
            Direction::In | Direction::To => "IN",
        };
        Ok(json!({
            "@type" : DIRECTION,
            "@value" : direction_str,
        }))
    }
}
