use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<P, D> for GraphSON<V2> {
    fn deserialize(_val: &Value) -> Result<P, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<P, D> for GraphSON<V2> {
    fn serialize(val: &P) -> Result<Value, Error> {
        Ok(json!({
            "predicate": val.operator,
            "value": (*val.value).serialize::<Self, D>()?
        }))
    }
}
