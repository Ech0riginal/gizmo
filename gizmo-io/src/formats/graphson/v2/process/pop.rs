use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Pop, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Pop, Error> {
        let repr = val.deserialize::<Self, D, String>()?;
        match repr.as_str() {
            Pop::ALL => Ok(Pop::All),
            Pop::FIRST => Ok(Pop::First),
            Pop::LAST => Ok(Pop::Last),
            Pop::MIXED => Ok(Pop::Mixed),
            _ => Err(Error::unexpected(val, "a valid Pop value")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Pop, D> for GraphSON<V2> {
    fn serialize(val: &Pop) -> Result<Value, Error> {
        Ok(json!(match val {
            Pop::All => Pop::ALL,
            Pop::First => Pop::FIRST,
            Pop::Last => Pop::LAST,
            Pop::Mixed => Pop::MIXED,
        }))
    }
}
