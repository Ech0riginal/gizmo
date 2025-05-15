use crate::io::graphson::prelude::*;

impl Deserializer<GID> for V2 {
    fn deserialize(val: &Value) -> Result<GID, Error> {
        let gvalue = val.deserialize::<Self, GValue>()?;
        match gvalue {
            GValue::String(d) => Ok(GID::String(d)),
            GValue::Integer(d) => Ok(GID::Integer(d)),
            GValue::Long(d) => Ok(GID::Long(d)),
            value => Err(Error::UnexpectedGValue {
                msg: "Ineligible for GKey".into(),
                value,
            }),
        }
    }
}

impl Deserializer<GID> for V3 {
    fn deserialize(val: &Value) -> Result<GID, Error> {
        <V2 as Deserializer<GID>>::deserialize(val)
    }
}

impl Serializer<GID> for V2 {
    fn serialize(val: &GID) -> Result<Value, Error> {
        let val: GValue = val.into();
        val.serialize::<Self>()
    }
}

impl Serializer<GID> for V3 {
    fn serialize(val: &GID) -> Result<Value, Error> {
        <V2 as Serializer<GID>>::serialize(val)
    }
}
