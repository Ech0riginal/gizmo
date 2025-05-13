use crate::GValue;
use crate::io::{Deserialize, Deserializer, Error, V2};
use crate::structure::GID;
use serde_json::Value;

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
