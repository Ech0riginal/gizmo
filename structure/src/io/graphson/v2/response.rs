use crate::io::graphson::prelude::*;
use crate::io::{Response, Status};

use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

impl Deserializer<Response> for V2 {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        let id = {
            let _id = V2::get(value, "requestId")?.clone();
            _id.deserialize::<Self, Uuid>()?
        };
        let result = Self::get(value, "result")?;
        let data = Self::get(result, "data")?.deserialize::<Self, GValue>()?;
        let meta = Self::get(result, "meta")?;
        let meta = get_value!(meta, Value::Object)?
            .into_iter()
            .map(|(k, v)| (k.clone(), v.deserialize::<Self, GValue>()))
            .map(|(k, result)| match result {
                Ok(v) => Ok((k, v)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<(String, GValue)>, Error>>()?
            .into_iter()
            .collect::<HashMap<_, _>>();

        let status = {
            let status = Self::get(value, "status")?;
            status.deserialize::<Self, Status>()?
        };

        Ok(Response {
            id,
            status,
            data,
            meta,
        })
    }
}
impl Serializer<Response> for V2 {
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

impl Deserializer<Status> for V2 {
    fn deserialize(val: &Value) -> Result<Status, Error> {
        let code = Self::get(val, "code").map(|code| code.as_i64().unwrap() as i16)?;
        let message = {
            let tmp = Self::get(val, "message")?;
            let str = get_value!(tmp, Value::String)?;
            if str.is_empty() {
                None
            } else {
                Some(str.clone())
            }
        };
        let attributes = Self::get(val, "attributes")?.clone();

        Ok(Status {
            code,
            message,
            attributes,
        })
    }
}
impl Serializer<Status> for V2 {
    fn serialize(val: &Status) -> Result<Value, Error> {
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
