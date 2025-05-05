use crate::io::{Deserializer, GremlinIO, IOHelpers, Serializer, V3};
use crate::message::{Message, Request, Response, Status};
use crate::{GValue, GremlinError, GremlinResult};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

pub(crate) mod de;
pub(crate) mod ser;
pub(crate) mod types;

#[cfg(test)]
mod tests;

crate::io::macros::io!(V2);

impl GremlinIO for V2 {
    fn mime() -> &'static str {
        "application/vnd.gremlin-v2.0+json"
    }
}

impl IOHelpers for V2 {}

impl Deserializer<Response> for V2 {
    fn deserialize(value: &Value) -> GremlinResult<Response> {
        let id = {
            let _id = Self::get(value, "request_id")?.clone();
            serde_json::from_value::<Uuid>(_id)?
        };
        let result = {
            let data = Self::get(Self::get(value, "result")?, "data")?;
            <Self as Deserializer<GValue>>::deserialize(data)?
        };
        let status = {
            let status = Self::get(value, "status")?;
            <Self as Deserializer<Status>>::deserialize(status)?
        };

        Ok(Response { id, result, status })
    }
}

impl Deserializer<GValue> for V2 {
    fn deserialize(value: &Value) -> GremlinResult<GValue> {
        de::deserialize::<Self>(value)
    }
}

impl Deserializer<Status> for V2 {
    fn deserialize(value: &Value) -> GremlinResult<Status> {
        let code = Self::get(value, "code").map(|code| code.as_i64().unwrap() as i16)?;
        let message = Self::get(value, "message")
            .ok()
            .map(|value| value.as_str().unwrap().to_string());

        Ok(Status { code, message })
    }
}

impl Serializer<GValue> for V2 {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        ser::serialize::<Self>(value)
    }
}

impl Serializer<Request> for V2 {
    fn serialize(value: &Request) -> GremlinResult<Value> {
        Ok(json!({
            "request_id": value.id,
            "op": value.op,
            "processor": value.proc,
            "args": value.args,
        }))
    }
}
