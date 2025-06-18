use crate::graphson::prelude::*;
use crate::{Args, Request};

impl<D: Dialect> GraphsonSerializer<Request, D> for GraphSON<V2> {
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "requestId": val.id.serialize::<Self, D>()?,
            "op": val.op,
            "processor": val.proc,
            "args": {
                "@type": <Map::<(),()> as Tag_<D>>::tag,
                "@value": val.args.0.serialize::<Self, D>()?,
            },
        }))
    }
}
