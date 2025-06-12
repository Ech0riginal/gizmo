use crate::graphson::prelude::*;

impl Deserializer<Traverser> for V2 {
    fn deserialize(_val: &Value) -> Result<Traverser, Error> {
        todo!()
    }
}
impl Serializer<Traverser> for V2 {
    fn serialize(val: &Traverser) -> Result<Value, Leaf> {
        todo!()
    }
}
