use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Operator, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Operator, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_ref() {
            "sum" => Ok(Operator::Sum),
            "minus" => Ok(Operator::Minus),
            "mult" => Ok(Operator::Mult),
            "div" => Ok(Operator::Div),
            "min" => Ok(Operator::Min),
            "max" => Ok(Operator::Max),
            "assign" => Ok(Operator::Assign),
            "and" => Ok(Operator::And),
            "or" => Ok(Operator::Or),
            "addAll" => Ok(Operator::AddAll),
            "sumLong" => Ok(Operator::SumLong),
            _ => Err(Error::unexpected(string, "a valid Operator")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Operator, D> for GraphSON<V3> {
    fn serialize(val: &Operator) -> Result<Value, Error> {
        Ok(json!(match val {
            Operator::Sum => "sum",
            Operator::Minus => "minus",
            Operator::Mult => "mult",
            Operator::Div => "div",
            Operator::Min => "min",
            Operator::Max => "max",
            Operator::Assign => "assign",
            Operator::And => "and",
            Operator::Or => "or",
            Operator::AddAll => "addAll",
            Operator::SumLong => "sumLong",
        }))
    }
}
