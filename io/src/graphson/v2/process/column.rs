use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Column, D> for GraphSON<V2> {
    fn deserialize(_val: &Value) -> Result<Column, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Column, D> for GraphSON<V2> {
    fn serialize(val: &Column) -> Result<Value, Error> {
        Ok(json!(match val {
            Column::Keys => "keys",
            Column::Values => "values",
        }))
    }
}
