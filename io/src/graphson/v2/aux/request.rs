use crate::graphson::prelude::*;
use crate::{Args, Request};

impl<D: Dialect> GraphsonSerializer<Request, D> for GraphSON<V2> {
    fn serialize(val: &Request) -> Result<Value, Error> {
        Ok(json!({
            "requestId": {
                "@type": D::tag::<Uuid>(),
                "@value": val.id.serialize::<Self, D>()?,
            },
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
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            GValue::Map(map) => map
                                .iter()
                                .map(|(k, v)| (k.serialize::<Self, D>(), v.serialize::<Self, D>()))
                                .map(|(kr, vr)| match kr {
                                    Ok(k) => match vr {
                                        Ok(v) => Ok((k, v)),
                                        Err(e) => Err(e),
                                    },
                                    Err(ke) => Err(ke),
                                })
                                .collect::<Result<IndexMap<_, _>, Error>>()
                                .map(|tmp| json!(tmp)),
                            _ => v.serialize::<Self, D>(),
                        },
                    )
                })
                .map(|(k, result)| match result {
                    Ok(v) => Ok((k, v)),
                    Err(e) => Err(e),
                })
                .collect::<Result<IndexMap<_, _>, Error>>()?
                .drain(..),
        );

        Ok(json!(map))
    }
}
