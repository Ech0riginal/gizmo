use crate::graphson::prelude::*;
use crate::{Response, Status};

use serde_json::Value;

impl Deserializer<Response> for V3 {
    fn deserialize(value: &Value) -> Result<Response, Leaf> {
        let map = get_value!(value, Value::Object).ctx::<Response>()?;
        let id = map
            .ensure("requestId")
            .ctx::<Response>()?
            .deserialize::<Self, uuid::Uuid>()
            .ctx::<Response>()?;
        let result = map.ensure("result").ctx::<Response>()?;
        let data = result
            .ensure("data")?
            .deserialize::<Self, GValue>()
            .ctx::<Response>()?;
        let meta = result.ensure("meta").ctx::<Response>()?;
        let meta = get_value!(meta, Value::Object)
            .ctx::<Response>()?
            .into_iter()
            .map(|(k, v)| (k.clone(), v.deserialize::<Self, GValue>().ctx::<Response>()))
            .map(|(k, result)| match result {
                Ok(v) => Ok((k, v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Map<String, GValue>, Leaf>>()?;
        let status = value
            .ensure("status")?
            .deserialize::<Self, Status>()
            .ctx::<Response>()?;
        Ok(Response {
            id,
            status,
            data,
            meta,
        })
    }
}

impl Deserializer<Status> for V3 {
    fn deserialize(val: &Value) -> Result<Status, Leaf> {
        let code = val
            .ensure("code")
            .map(|code| code.as_i64().unwrap() as i16)
            .ctx::<Status>()?;
        let message = {
            let tmp = val.ensure("message").ctx::<Status>()?;
            let str = get_value!(tmp, Value::String).ctx::<Status>()?;
            if str.is_empty() {
                None
            } else {
                Some(str.clone())
            }
        };
        let attributes = val.ensure("attributes").ctx::<Status>()?.clone();

        Ok(Status {
            code,
            message,
            attributes,
        })
    }
}

impl Serializer<Response> for V3 {
    fn serialize(val: &Response) -> Result<Value, Leaf> {
        let mut meta = IndexMap::new();

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
    fn serialize(val: &Status) -> Result<Value, Leaf> {
        let message = if let Some(msg) = &val.message {
            msg
        } else {
            ""
        };

        Ok(json!({
            "code": val.code,
            "message": message,
            "attributes": val.attributes,
        }))
    }
}
