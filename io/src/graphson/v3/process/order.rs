use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Order, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Order, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Order, D> for GraphSON<V3> {
    fn serialize(val: &Order) -> Result<Value, Error> {
        todo!()
    }
}
