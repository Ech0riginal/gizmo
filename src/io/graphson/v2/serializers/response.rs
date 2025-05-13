use crate::Response;
use crate::io::{Error, GResult, Serialize, Serializer, Status, V2};
use serde_json::{Value, json};
use std::collections::HashMap;

impl Serializer<Response> for V2 {
    fn serialize(val: &Response) -> Result<Value, Error> {
        Ok(json!({
            "requestId": val.id,
            "result": val.result.serialize::<Self>()?,
            "status": val.status.serialize::<Self>()?,
        }))
    }
}
impl Serializer<GResult> for V2 {
    fn serialize(val: &GResult) -> Result<Value, Error> {
        let mut meta = HashMap::new();

        for (key, value) in val.meta.iter() {
            let serialized = value.serialize::<Self>()?;
            meta.insert(Value::String(key.clone()), serialized);
        }

        Ok(json!({
            "data": val.data.serialize::<Self>()?,
            "meta": meta,
        }))
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
