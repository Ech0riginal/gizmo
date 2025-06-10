use crate::io::graphson::prelude::*;

passthrough!(String, V3 to V2);

impl<'a> Serializer<&'a str> for V3 {
    fn serialize(val: &&'a str) -> Result<Value, Error> {
        Ok(json!(val))
    }
}
