use crate::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<TextP, D> for GraphSON<V2> {
    fn deserialize(_val: &Value) -> Result<TextP, Error> {
        todo!()
    }
}

impl<D: Dialect> GraphsonSerializer<TextP, D> for GraphSON<V2> {
    fn serialize(val: &TextP) -> Result<Value, Error> {
        Ok(json!({
            "predicate" : val.operator(),
            "value" : val.value().serialize::<Self, D>()?
        }))
    }
}
