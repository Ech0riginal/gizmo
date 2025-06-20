use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Direction, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Direction, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Direction, D> for GraphSON<V3> {
    fn serialize(_val: &Direction) -> Result<Value, Error> {
        todo!()
    }
}
