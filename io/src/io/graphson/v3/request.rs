use crate::io::graphson::prelude::*;
use crate::{Args, Request};

impl Deserializer<Request> for V3 {
    fn deserialize(val: &Value) -> Result<Request, Error> {
        todo!()
    }
}
impl Serializer<Request> for V3 {
    fn serialize(val: &Request) -> Result<Value, Error> {
        let argh = val.args.serialize::<Self>()?;
        Ok(json!({
            "requestId": val.id,
            "op": val.op,
            "processor": val.proc,
            "args": argh
        }))
    }
}
impl Serializer<Args> for V3 {
    fn serialize(value: &Args) -> Result<Value, Error> {
        value.0.serialize::<Self>()
    }
}
