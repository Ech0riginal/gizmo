use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Merge, D> for GraphSON<V2> {
    fn deserialize(_val: &Value) -> Result<Merge, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Merge, D> for GraphSON<V2> {
    fn serialize(_val: &Merge) -> Result<Value, Error> {
        todo!()
    }
}
