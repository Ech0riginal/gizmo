use crate::graphson::prelude::*;

impl Deserializer<Bytecode> for V3 {
    fn deserialize(val: &Value) -> Result<Bytecode, Error> {
        todo!()
    }
}

impl Serializer<Bytecode> for V3 {
    fn serialize(val: &Bytecode) -> Result<Value, Error> {
        todo!()
    }
}
