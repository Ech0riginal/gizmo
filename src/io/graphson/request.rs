use crate::io::graphson::prelude::*;
use crate::io::{Args, Request};

impl Deserializer<Request> for V2 {
    fn deserialize(val: &Value) -> Result<Request, Error> {
        todo!()
    }
}

impl Deserializer<Request> for V3 {
    fn deserialize(val: &Value) -> Result<Request, Error> {
        todo!()
    }
}

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
        let mut map = HashMap::new();
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

impl Serializer<Request> for V3 {
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "requestId": val.id.serialize::<Self>()?,
            "op": val.op,
            "processor": val.proc,
            "args": val.args.serialize::<Self>()?,
        }))
    }
}

impl Serializer<Args> for V3 {
    fn serialize(value: &Args) -> Result<Value, Error> {
        let mut map = HashMap::new();
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
