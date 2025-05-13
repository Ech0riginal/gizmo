use crate::GValue;
use crate::io::{Error, Serialize, Serializer, V2};
use crate::structure::GID;
use serde_json::Value;

impl Serializer<GID> for V2 {
    fn serialize(val: &GID) -> Result<Value, Error> {
        let val: GValue = val.into();
        val.serialize::<Self>()
    }
}
