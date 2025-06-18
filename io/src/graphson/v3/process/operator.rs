use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Operator, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Operator, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Operator, D> for GraphSON<V3> {
    fn serialize(_val: &Operator) -> Result<Value, Error> {
        todo!()
    }
}
