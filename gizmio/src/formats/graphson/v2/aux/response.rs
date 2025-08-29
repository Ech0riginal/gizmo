use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::formats::graphson::prelude::*;
use crate::{Response, Status};

impl<D: Dialect> GraphsonDeserializer<Response, D> for GraphSON<V2> {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        let map = get_value!(value, Value::Object)?;
        let id = map.ensure("requestId")?.deserialize::<Self, D, Uuid>()?;
        let result = map.ensure("result")?;
        let data = result.ensure("data")?.deserialize::<Self, D, GValue>()?;
        let meta = result.ensure("meta")?;
        let meta = get_value!(meta, Value::Object)?
            .into_iter()
            .map(|(k, v)| (k.clone(), v.deserialize::<Self, D, GValue>()))
            .map(|(k, result)| match result {
                Ok(v) => Ok((k, v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Map<String, GValue>, Error>>()?;
        let status = value.ensure("status")?.deserialize::<Self, D, Status>()?;
        Ok(Response {
            id,
            status,
            data,
            meta,
        })
    }
}
impl<D: Dialect> GraphsonSerializer<Response, D> for GraphSON<V2> {
    fn serialize(val: &Response) -> Result<Value, Error> {
        let mut meta = HashMap::new();

        for (key, value) in val.meta.iter() {
            let serialized = value.serialize::<Self, D>()?;
            meta.insert(Value::String(key.clone()), serialized);
        }

        Ok(json!({
            "requestId": val.id,
            "result": {
                "data": val.data.serialize::<Self, D>()?,
                "meta": meta,
            },
            "status": val.status.serialize::<Self, D>()?,
        }))
    }
}

impl<D: Dialect> GraphsonDeserializer<Status, D> for GraphSON<V2> {
    fn deserialize(val: &Value) -> Result<Status, Error> {
        let code = val
            .ensure("code")
            .map(|code| code.as_i64().unwrap() as i16)?
            .into();
        let message = {
            let tmp = val.ensure("message")?;
            let str = get_value!(tmp, Value::String)?;
            if str.is_empty() {
                None
            } else {
                Some(str.clone())
            }
        };
        let attributes = val.ensure("attributes")?.clone();

        Ok(Status {
            code,
            message,
            attributes,
        })
    }
}
impl<D: Dialect> GraphsonSerializer<Status, D> for GraphSON<V2> {
    fn serialize(val: &Status) -> Result<Value, Error> {
        let message = if let Some(msg) = &val.message {
            msg
        } else {
            ""
        };

        Ok(json!({
            "code": val.code.i16(),
            "message": message,
            "attributes": val.attributes,
        }))
    }
}
