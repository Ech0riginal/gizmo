use crate::graphson::prelude::*;
use crate::{Args, Request};

impl Serializer<Request> for V2 {
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "requestId": val.id.serialize::<Self>()?,
            "op": val.op,
            "processor": val.proc,
            "args": val.args.serialize::<Self>()?,
        }))
    }
}
impl Serializer<Args> for V2 {
    fn serialize(value: &Args) -> Result<Value, Error> {
        let mut map = IndexMap::new();
        map.extend(
            value
                .iter()
                .map(|(k, v)| (k, v.serialize::<Self>()))
                .map(|(k, result)| match result {
                    Ok(v) => Ok((*k, v)),
                    Err(e) => Err(e),
                })
                .collect::<Result<Vec<(&'static str, Value)>, Error>>()?
                .drain(..),
        );

        Ok(json!(map))
    }
}
