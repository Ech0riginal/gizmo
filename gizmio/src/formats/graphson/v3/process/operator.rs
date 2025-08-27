use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Operator, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Operator, Error> {
        let string = get_value!(val, Value::String)?;
        match string.as_ref() {
            Operator::SUM => Ok(Operator::Sum),
            Operator::MINUS => Ok(Operator::Minus),
            Operator::MULT => Ok(Operator::Mult),
            Operator::DIV => Ok(Operator::Div),
            Operator::MIN => Ok(Operator::Min),
            Operator::MAX => Ok(Operator::Max),
            Operator::ASSIGN => Ok(Operator::Assign),
            Operator::AND => Ok(Operator::And),
            Operator::OR => Ok(Operator::Or),
            Operator::ADD_ALL => Ok(Operator::AddAll),
            Operator::SUM_LONG => Ok(Operator::SumLong),
            _ => Err(Error::unexpected(string, "a valid Operator")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Operator, D> for GraphSON<V3> {
    fn serialize(val: &Operator) -> Result<Value, Error> {
        Ok(json!(match val {
            Operator::Sum => Operator::SUM,
            Operator::Minus => Operator::MINUS,
            Operator::Mult => Operator::MULT,
            Operator::Div => Operator::DIV,
            Operator::Min => Operator::MIN,
            Operator::Max => Operator::MAX,
            Operator::Assign => Operator::ASSIGN,
            Operator::And => Operator::AND,
            Operator::Or => Operator::OR,
            Operator::AddAll => Operator::ADD_ALL,
            Operator::SumLong => Operator::SUM_LONG,
        }))
    }
}
