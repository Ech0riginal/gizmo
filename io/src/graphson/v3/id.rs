use crate::graphson::prelude::*;

impl Deserializer<GID> for V3 {
    fn deserialize(val: &Value) -> Result<GID, Leaf> {
        let gvalue = val.deserialize::<Self, GValue>().ctx::<GID>()?;
        match gvalue {
            GValue::String(d) => Ok(GID::String(d)),
            GValue::Integer(d) => Ok(GID::Integer(d)),
            GValue::Long(d) => Ok(GID::Long(d)),
            value => Err(Leaf::Unexpected {
                expectation: "an eligible GKey".to_string(),
                actual: format!("{:?}", value),
                location: location!(),
            }),
        }
    }
}

impl Serializer<GID> for V3 {
    fn serialize(val: &GID) -> Result<Value, Leaf> {
        let val: GValue = val.into();
        val.serialize::<Self>()
    }
}
