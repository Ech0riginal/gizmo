use crate::graphson::prelude::*;

impl Deserializer<Binding> for V3 {
    fn deserialize(val: &Value) -> Result<Binding, Error> {
        todo!()
    }
}

impl Serializer<Binding> for V3 {
    fn serialize(val: &Binding) -> Result<Value, Error> {
        todo!()
    }
}
