use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Pick, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Pick, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Pick, D> for GraphSON<V3> {
    fn serialize(val: &Pick) -> Result<Value, Error> {
        todo!()
    }
}
