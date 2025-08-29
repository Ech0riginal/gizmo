use crate::formats::graphson::prelude::*;
use crate::{Args, Request};

impl<D: Dialect> GraphsonSerializer<Request, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "requestId": val.id,
            "op": val.op,
            "processor": val.proc,
            "args": {
                "@type": <Map::<(),()> as AST<D>>::tag,
                "@value": val.args.serialize::<Self, D>()?,
            },
        }))
    }
}
impl<D: Dialect> GraphsonSerializer<Args, D> for GraphSON<V3>
where
    Self: GraphsonSerializer<GValue, D>,
{
    fn serialize(value: &Args) -> Result<Value, Error> {
        value.0.serialize::<Self, D>()
    }
}

impl Named for &'static str {
    const name: &'static str = "not an object";
}
