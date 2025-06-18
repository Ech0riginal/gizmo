use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Order, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Order, Error> {
        todo!()
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
