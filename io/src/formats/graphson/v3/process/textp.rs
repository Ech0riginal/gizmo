use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TextP, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<TextP, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<TextP, D> for GraphSON<V3> {
    fn serialize(_val: &TextP) -> Result<Value, Error> {
        todo!()
    }
}
