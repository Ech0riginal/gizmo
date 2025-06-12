use crate::graphson::prelude::*;

impl Deserializer<GID> for V3 {
    fn deserialize(val: &Value) -> Result<GID, Error> {
        let gvalue = val.deserialize::<Self, GValue>()?;
        match gvalue {
            GValue::String(d) => Ok(GID::String(d)),
            GValue::Integer(d) => Ok(GID::Integer(d)),
            GValue::Long(d) => Ok(GID::Long(d)),
            value => Err(Error::Unexpected {
                expectation: "an eligible GKey".to_string(),
                actual: format!("{value:?}"),
                location: location!(),
            }),
        }
    }
}

impl Serializer<GID> for V3 {
    fn serialize(val: &GID) -> Result<Value, Error> {
        let val: GValue = val.into();
        val.serialize::<Self>()
    }
}
