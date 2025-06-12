use crate::graphson::prelude::*;

impl Deserializer<Column> for V3 {
    fn deserialize(val: &Value) -> Result<Column, Error> {
        todo!()
    }
}

impl Serializer<Column> for V3 {
    fn serialize(val: &Column) -> Result<Value, Error> {
        todo!()
    }
}
