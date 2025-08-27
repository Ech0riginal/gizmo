use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Pick, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Pick, Error> {
        let repr = val.deserialize::<Self, D, String>()?;
        match repr.as_str() {
            Pick::ANY => Ok(Pick::Any),
            Pick::NONE => Ok(Pick::None),
            _ => Err(Error::unexpected(val, "'any' or 'none'")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Pick, D> for GraphSON<V3> {
    fn serialize(val: &Pick) -> Result<Value, Error> {
        Ok(json!(match val {
            Pick::Any => Pick::ANY,
            Pick::None => Pick::NONE,
        }))
    }
}
