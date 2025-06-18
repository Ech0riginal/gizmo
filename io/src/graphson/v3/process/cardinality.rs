use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Cardinality, D> for GraphSON<V3> {
    fn deserialize(_val: &Value) -> Result<Cardinality, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Cardinality, D> for GraphSON<V3> {
    fn serialize(_val: &Cardinality) -> Result<Value, Error> {
        todo!()
    }
}
