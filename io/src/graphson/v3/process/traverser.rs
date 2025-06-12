use crate::graphson::prelude::*;

impl Deserializer<Traverser> for V3 {
    fn deserialize(val: &Value) -> Result<Traverser, Error> {
        todo!()
    }
}

impl Serializer<Traverser> for V3 {
    fn serialize(val: &Traverser) -> Result<Value, Error> {
        todo!()
    }
}

