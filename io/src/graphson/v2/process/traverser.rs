use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Traverser, D> for GraphSON<V2> {
    fn deserialize(_val: &Value) -> Result<Traverser, Error> {
        todo!()
    }
}
impl<D: Dialect> GraphsonSerializer<Traverser, D> for GraphSON<V2> {
    fn serialize(val: &Traverser) -> Result<Value, Error> {
        todo!()
    }
}
