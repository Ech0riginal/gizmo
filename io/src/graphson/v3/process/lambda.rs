use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Lambda, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Lambda, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Lambda, D> for GraphSON<V3> {
    fn serialize(_val: &Lambda) -> Result<Value, Error> {
        todo!()
    }
}
