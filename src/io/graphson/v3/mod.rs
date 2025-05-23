use crate::io::{Deserializer, Error, GremlinIO, Request, Response, Serializer};
use crate::{GValue, GremlinResult};
use serde_json::Value;

pub(crate) mod de;
pub(crate) mod ser;
#[cfg(test)]
mod tests;
pub(crate) mod types;

crate::io::macros::io!(V3);

impl GremlinIO for V3 {
    const version: &'static str = "V3";
    fn mime() -> &'static str {
        "application/vnd.gremlin-v3.0+json;types=true"
    }
}

impl Deserializer<Response> for V3 {
    fn deserialize(value: &Value) -> Result<Response, Error> {
        todo!()
    }
}

impl Deserializer<GValue> for V3 {
    fn deserialize(value: &Value) -> Result<GValue, Error> {
        todo!()
    }
}

impl Serializer<Request> for V3 {
    fn serialize(value: &Request) -> Result<Value, Error> {
        todo!()
    }
}

impl Serializer<GValue> for V3 {
    fn serialize(value: &GValue) -> Result<Value, Error> {
        todo!()
    }
}

// impl Gremlin for V3 {
//     fn mime() -> &'static str {
//         "application/vnd.gremlin-v3.0+json;types=true"
//     }
//
//     fn deserialize(value: &serde_json::Value) -> crate::Result<crate::GValue, Error> {
//         de::deserialize::<Self>(value)
//     }
//
//     fn serialize(value: &crate::GValue) -> crate::Result<serde_json::Value, Error> {
//         ser::serialize::<Self>(value)
//     }
//
// fn message<T>(op: String, processor: String, args: T, id: Option<uuid::Uuid>) -> crate::message::Message<T> {
//     let request_id = id.unwrap_or_else(uuid::Uuid::new_v4);
//     crate::message::Message::V3 {
//         request_id,
//         op,
//         processor,
//         args,
//     }
// }
// }

pub struct Viss;
