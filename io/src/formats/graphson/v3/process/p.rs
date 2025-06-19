use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<P, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<P, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<P, D> for GraphSON<V3> {
    fn serialize(_val: &P) -> Result<Value, Error> {
        todo!()
    }
}
