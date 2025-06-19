use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Scope, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Scope, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<Scope, D> for GraphSON<V2> {
    fn serialize(val: &Scope) -> Result<Value, Error> {
        Ok(json!(match val {
            Scope::Global => "global",
            Scope::Local => "local",
        }))
    }
}
