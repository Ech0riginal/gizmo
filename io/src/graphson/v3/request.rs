use crate::graphson::prelude::*;
use crate::{Args, Request};

impl Deserializer<Request> for V3 {
    fn deserialize(_val: &Value) -> Result<Request, Error> {
        todo!()
    }
}
impl Serializer<Request> for V3 {
    fn serialize(val: &Request) -> Result<Value, Error> {
        let argh = val.args.serialize::<Self>().ctx::<Request>()?;
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
        value.0.serialize::<Self>().ctx::<Args>()
    }
}

impl Object for &'static str {
    const name: &'static str = "not an object";
}
