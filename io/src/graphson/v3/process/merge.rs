use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Merge, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Merge, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Merge, D> for GraphSON<V3> {
    fn serialize(val: &Merge) -> Result<Value, Error> {
        todo!()
    }
}
