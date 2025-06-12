use crate::graphson::prelude::*;

impl Deserializer<TextP> for V3 {
    fn deserialize(val: &Value) -> Result<TextP, Error> {
        todo!()
    }
}

impl Serializer<TextP> for V3 {
    fn serialize(val: &TextP) -> Result<Value, Error> {
        todo!()
    }
}
