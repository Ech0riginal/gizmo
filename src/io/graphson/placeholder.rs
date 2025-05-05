//! A virtual placeholder for when we initialize the client.

use crate::io::{Deserializer, GremlinIO, Request, Response, Serializer};
use crate::{GValue, GremlinResult};
use serde_json::Value;

impl GremlinIO for () {
    fn mime() -> &'static str {
        "none"
    }
}

impl Deserializer<Response> for () {
    fn deserialize(value: &Value) -> GremlinResult<Response> {
        todo!()
    }
}

impl Deserializer<GValue> for () {
    fn deserialize(value: &Value) -> GremlinResult<GValue> {
        todo!()
    }
}

impl Serializer<Request> for () {
    fn serialize(value: &Request) -> GremlinResult<Value> {
        todo!()
    }
}

impl Serializer<GValue> for () {
    fn serialize(value: &GValue) -> GremlinResult<Value> {
        todo!()
    }
}
