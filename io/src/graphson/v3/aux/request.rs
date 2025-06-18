use crate::graphson::prelude::*;
use crate::{Args, Request};

impl<D: Dialect> GraphsonSerializer<Request, D> for GraphSON<V3> {
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
impl<D: Dialect> GraphsonSerializer<Args, D> for GraphSON<V3> {
    fn serialize(value: &Args) -> Result<Value, Error> {
        value.0.serialize::<Self>()
    }
}

impl Object for &'static str {
    const name: &'static str = "not an object";
}
