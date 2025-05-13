use crate::io::{Deserialize, Deserializer, Error, GResult, IOHelpers, Status, V2, get_value};
use crate::{GValue, Response};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

impl Deserializer<Response> for V2 {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        let id = {
            let _id = Self::get(value, "requestId")?.clone();
            _id.deserialize::<Self, Uuid>()?
        };
        let result = {
            let result = Self::get(value, "result")?;
            result.deserialize::<Self, GResult>()?
        };
        let status = {
            let status = Self::get(value, "status")?;
            status.deserialize::<Self, Status>()?
        };

        Ok(Response { id, result, status })
    }
}

impl Deserializer<GResult> for V2 {
    fn deserialize(val: &Value) -> Result<GResult, Error> {
        let data = Self::get(val, "data")?.deserialize::<Self, GValue>()?;
        let meta = Self::get(val, "meta")?; //.deserialize::<Self, GValue>()?;
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

        Ok(GResult { data, meta })
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
