use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Order, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Order, Error> {
        match get_value!(val, Value::String)?.as_str() {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            "shuffle" => Ok(Order::Shuffle),
            item => Err(Error::unexpected(item, "a valid Order")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Order, D> for GraphSON<V2> {
    fn serialize(val: &Order) -> Result<Value, Error> {
        Ok(json!(match val {
            Order::Asc => "asc",
            Order::Desc => "desc",
            Order::Shuffle => "shuffle",
        }))
    }
}
