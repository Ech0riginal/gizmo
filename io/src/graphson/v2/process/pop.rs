use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Pop, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Pop, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Pop, D> for GraphSON<V2> {
    fn serialize(val: &Pop) -> Result<Value, Error> {
        Ok(json!(match val {
            Pop::All => "all",
            Pop::First => "first",
            Pop::Last => "last",
            Pop::Mixed => "mixed",
        }))
    }
}
