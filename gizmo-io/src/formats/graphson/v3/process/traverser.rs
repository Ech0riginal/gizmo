use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Traverser, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Traverser, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Traverser, D> for GraphSON<V3> {
    fn serialize(_val: &Traverser) -> Result<Value, Error> {
        todo!()
    }
}
