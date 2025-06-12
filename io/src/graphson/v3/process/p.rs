use crate::graphson::prelude::*;

impl Deserializer<P> for V3 {
    fn deserialize(val: &Value) -> Result<P, Error> {
        todo!()
    }
}

impl Serializer<P> for V3 {
    fn serialize(val: &P) -> Result<Value, Error> {
        todo!()
    }
}
