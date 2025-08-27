use crate::formats::graphson::prelude::*;

impl<D: Dialect> GraphsonDeserializer<GID, D> for GraphSON<V3>
where
    Self: GraphsonDeserializer<GValue, D>,
{
    fn deserialize(val: &Value) -> Result<GID, Error> {
        let gvalue = val.deserialize::<Self, D, GValue>()?;
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

impl<D: Dialect> GraphsonSerializer<GID, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &GID) -> Result<Value, Error> {
        let val: GValue = val.into();
        val.serialize::<Self, D>()
    }
}
