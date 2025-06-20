use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Pop, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Pop, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Pop, D> for GraphSON<V3> {
    fn serialize(_val: &Pop) -> Result<Value, Error> {
        todo!()
    }
}
