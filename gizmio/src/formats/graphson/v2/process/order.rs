use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Order, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Order, Error> {
        match get_value!(val, Value::String)?.as_str() {
            Order::ASC => Ok(Order::Asc),
            Order::DESC => Ok(Order::Desc),
            Order::SHUFFLE => Ok(Order::Shuffle),
            item => Err(Error::unexpected(item, "a valid Order")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Order, D> for GraphSON<V2> {
    fn serialize(val: &Order) -> Result<Value, Error> {
        Ok(json!(match val {
            Order::Asc => Order::ASC,
            Order::Desc => Order::DESC,
            Order::Shuffle => Order::SHUFFLE,
        }))
    }
}
