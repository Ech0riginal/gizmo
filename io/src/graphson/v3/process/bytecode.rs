use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Bytecode, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Bytecode, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Bytecode, D> for GraphSON<V3> {
    fn serialize(val: &Bytecode) -> Result<Value, Error> {
        todo!()
    }
}
