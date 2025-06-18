use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Operator, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Operator, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Operator, D> for GraphSON<V3> {
    fn serialize(val: &Operator) -> Result<Value, Error> {
        todo!()
    }
}
