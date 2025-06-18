use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Cardinality, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Cardinality, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Cardinality, D> for GraphSON<V3> {
    fn serialize(val: &Cardinality) -> Result<Value, Error> {
        todo!()
    }
}
