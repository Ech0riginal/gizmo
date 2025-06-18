use crate::graphson::prelude::*;
use crate::{Args, Request};

impl<D: Dialect> GraphsonSerializer<Request, D> for GraphSON<V2> {
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "requestId": val.id.serialize::<Self, D>()?,
            "op": val.op,
            "processor": val.proc,
            "args": val.args.serialize::<Self, D>()?,
        }))
    }
}
impl<D: Dialect> GraphsonSerializer<Args, D> for GraphSON<V2> {
    fn serialize(value: &Args) -> Result<Value, Error> {
        let mut map = IndexMap::new();
        map.extend(
            value
                .iter()
                .map(|(k, v)| (k, v.serialize::<Self, D>()))
                .map(|(k, result)| match result {
                    Ok(v) => Ok((k, v)),
                    Err(e) => Err(e),
                })
                .collect::<Result<List<_>, Error>>()?
                .drain(..),
        );

        Ok(json!(map))
    }
}
