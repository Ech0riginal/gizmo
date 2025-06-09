use crate::io::graphson::prelude::*;

impl Deserializer<TinkerGraph> for V3 {
    fn deserialize(val: &Value) -> Result<TinkerGraph, Error> {
        todo!()
    }
}

impl Serializer<TinkerGraph> for V3 {
    fn serialize(val: &TinkerGraph) -> Result<Value, Error> {
        todo!()
    }
}
