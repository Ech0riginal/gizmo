use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Pop, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Pop, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Pop, D> for GraphSON<V3> {
    fn serialize(val: &Pop) -> Result<Value, Error> {
        todo!()
    }
}
