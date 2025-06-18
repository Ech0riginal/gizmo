use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TextP, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<TextP, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<TextP, D> for GraphSON<V3> {
    fn serialize(val: &TextP) -> Result<Value, Error> {
        todo!()
    }
}
