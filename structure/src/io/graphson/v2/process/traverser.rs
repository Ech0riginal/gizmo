use crate::io::graphson::prelude::*;

impl Deserializer<Traverser> for V2 {
    fn deserialize(_val: &Value) -> Result<Traverser, Error> {
        todo!()
    }
}
impl Serializer<Traverser> for V2 {
    fn serialize(val: &Traverser) -> Result<serde_json::Value, Error> {
        todo!()
    }
}
