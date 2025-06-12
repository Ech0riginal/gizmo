use crate::graphson::prelude::*;

impl Deserializer<Order> for V3 {
    fn deserialize(val: &Value) -> Result<Order, Error> {
        todo!()
    }
}

impl Serializer<Order> for V3 {
    fn serialize(val: &Order) -> Result<Value, Error> {
        todo!()
    }
}
