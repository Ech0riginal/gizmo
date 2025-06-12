use crate::graphson::prelude::*;

impl Deserializer<Lambda> for V3 {
    fn deserialize(val: &Value) -> Result<Lambda, Error> {
        todo!()
    }
}

impl Serializer<Lambda> for V3 {
    fn serialize(val: &Lambda) -> Result<Value, Error> {
        todo!()
    }
}
