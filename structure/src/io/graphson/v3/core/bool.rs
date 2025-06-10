use crate::io::graphson::prelude::*;

impl Deserializer<Bool> for V3 {
    fn deserialize(val: &Value) -> Result<Bool, Error> {
        <V2 as Deserializer<Bool>>::deserialize(val)
    }
}
impl Serializer<Bool> for V3 {
    fn serialize(val: &Bool) -> Result<Value, Error> {
        <V2 as Serializer<Bool>>::serialize(val)
    }
}
