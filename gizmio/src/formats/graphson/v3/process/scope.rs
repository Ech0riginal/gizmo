use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<Scope, D> for GraphSON<V3> {
    fn deserialize(val: &Value) -> Result<Scope, Error> {
        let repr = val.deserialize::<Self, D, String>()?;
        match repr.as_str() {
            Scope::GLOBAL => Ok(Scope::Global),
            Scope::LOCAL => Ok(Scope::Local),
            _ => Err(Error::unexpected(val, "'global' or 'local'")),
        }
    }
}

impl<D: Dialect> GraphsonSerializer<Scope, D> for GraphSON<V3> {
    fn serialize(val: &Scope) -> Result<Value, Error> {
        Ok(json!(match val {
            Scope::Global => Scope::GLOBAL,
            Scope::Local => Scope::LOCAL,
        }))
    }
}
