use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Binding, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Binding, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Binding, D> for GraphSON<V3> {
    fn serialize(_val: &Binding) -> Result<Value, Error> {
        todo!()
    }
}
