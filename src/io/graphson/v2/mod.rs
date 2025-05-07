use crate::io::{Args, Deserializer, GremlinIO, IOHelpers, Request, Response, Serializer, Status};
use crate::{GValue, GremlinResult};
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
        let id: Uuid = {
            let _id = Self::get(value, "request_id")?.clone();
            serde_json::from_value(_id)?
        };
        let result: GValue = {
            let result = Self::get(value, "result")?;
            let data = Self::get(result, "data")?;
            Self::deserialize(data)?
        };
        let status: Status = {
            let status = Self::get(value, "status")?;
            Self::deserialize(status)?
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

impl Serializer<Request> for V2 {
    fn serialize(value: &Request) -> GremlinResult<Value> {
        Ok(json!({
            "request_id": value.id,
            "op": value.op,
            "processor": value.proc,
            "args": Self::serialize(&value.args)?,
        }))
    }
}

impl Serializer<Args> for V2 {
    fn serialize(value: &Args) -> GremlinResult<Value> {
        todo!()
    }
}

impl Serializer<GValue> for V2 {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        ser::serialize::<Self>(value)
    }
}

fn a() {
    let y = 1481750076295;
}
