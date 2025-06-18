use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Scope, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Scope, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Scope, D> for GraphSON<V3> {
    fn serialize(val: &Scope) -> Result<Value, Error> {
        todo!()
    }
}
