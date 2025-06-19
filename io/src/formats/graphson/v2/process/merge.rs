use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Merge, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Merge, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Merge, D> for GraphSON<V2> {
    fn serialize(val: &Merge) -> Result<Value, Error> {
        todo!()
    }
}
