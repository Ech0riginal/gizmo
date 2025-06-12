use crate::graphson::prelude::*;

impl Deserializer<Scope> for V3 {
    fn deserialize(val: &Value) -> Result<Scope, Error> {
        todo!()
    }
}

impl Serializer<Scope> for V3 {
    fn serialize(val: &Scope) -> Result<Value, Error> {
        todo!()
    }
}
