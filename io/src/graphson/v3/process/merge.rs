use crate::graphson::prelude::*;

impl Deserializer<Merge> for V3 {
    fn deserialize(val: &Value) -> Result<Merge, Error> {
        todo!()
    }
}

impl Serializer<Merge> for V3 {
    fn serialize(val: &Merge) -> Result<Value, Error> {
        todo!()
    }
}
