use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Binding, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Binding, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Binding, D> for GraphSON<V3> {
    fn serialize(val: &Binding) -> Result<Value, Error> {
        todo!()
    }
}
