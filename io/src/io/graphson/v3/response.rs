use crate::io::graphson::prelude::*;
use crate::{Response, Status};

use serde_json::Value;

impl Deserializer<Response> for V3 {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        let map = get_value!(value, Value::Object)?;
        let id = map.ensure("requestId")?.deserialize::<Self, uuid::Uuid>()?;
        let result = map.ensure("result")?;
        let data = result.ensure("data")?.deserialize::<Self, GValue>()?;
        let meta = result.ensure("meta")?;
        let meta = get_value!(meta, Value::Object)?
            .into_iter()
            .map(|(k, v)| (k.clone(), v.deserialize::<Self, GValue>()))
            .map(|(k, result)| match result {
                Ok(v) => Ok((k, v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Map<String, GValue>, Error>>()?;
        let status = value.ensure("status")?.deserialize::<Self, Status>()?;
        Ok(Response {
            id,
            status,
            data,
            meta,
        })
    }
}

impl Deserializer<Status> for V3 {
    fn deserialize(val: &Value) -> Result<Status, Error> {
        <V2 as Deserializer<Status>>::deserialize(val)
    }
}

impl Serializer<Response> for V3 {
    fn serialize(val: &Response) -> Result<Value, Error> {
        let mut meta = HashMap::new();

        for (key, value) in val.meta.iter() {
            let serialized = value.serialize::<Self>()?;
            meta.insert(Value::String(key.clone()), serialized);
        }

        Ok(json!({
            "requestId": val.id,
            "result": {
                "data": val.data.serialize::<Self>()?,
                "meta": meta,
            },
            "status": val.status.serialize::<Self>()?,
        }))
    }
}

impl Serializer<Status> for V3 {
    fn serialize(val: &Status) -> Result<Value, Error> {
        <V2 as Serializer<Status>>::serialize(val)
    }
}
