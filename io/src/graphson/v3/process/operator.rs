use crate::graphson::prelude::*;

impl Deserializer<Operator> for V3 {
    fn deserialize(val: &Value) -> Result<Operator, Error> {
        todo!()
    }
}

impl Serializer<Operator> for V3 {
    fn serialize(val: &Operator) -> Result<Value, Error> {
        todo!()
    }
}
