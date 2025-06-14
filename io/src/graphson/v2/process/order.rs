use crate::graphson::prelude::*;

impl Serializer<Order> for V2 {
    fn serialize(val: &Order) -> Result<Value, Error> {
        let str = match val {
            Order::Asc => "asc",
            Order::Desc => "desc",
            Order::Shuffle => "shuffle",
        };
        Ok(json!({
            "@type": Tag::Order,
            "@value": str,
        }))
    }
}
