use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Bytecode, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Bytecode, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Bytecode, D> for GraphSON<V3> {
    fn serialize(_val: &Bytecode) -> Result<Value, Error> {
        todo!()
    }
}
