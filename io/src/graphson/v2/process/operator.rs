use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Operator, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Operator, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Operator, D> for GraphSON<V2> {
    fn serialize(val: &Operator) -> Result<Value, Error> {
        todo!()
    }
}
