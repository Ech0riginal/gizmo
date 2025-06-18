use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Column, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Column, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Column, D> for GraphSON<V3> {
    fn serialize(val: &Column) -> Result<Value, Error> {
        todo!()
    }
}
